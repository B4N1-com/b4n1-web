# frozen_string_literal: true

require "open3"
require "timeout"
require_relative "binary"
require_relative "page"
require_relative "errors"
require_relative "version"

module B4n1Web
  # Browser modes accepted by the binary.
  module BrowserMode
    LIGHT  = "light"
    JS     = "js"
    RENDER = "render"
  end

  # Thin async-free wrapper over the b4n1web binary.
  #
  #   browser = B4n1Web::AgentBrowser.new(mode: B4n1Web::BrowserMode::LIGHT)
  #   page = browser.goto("https://example.com")
  #   puts page.markdown
  #   browser.close
  class AgentBrowser
    attr_reader :mode, :timeout

    # runner: injectable process runner (object responding to
    #   run(argv, timeout:) -> [stdout, stderr, status_int]). Tests inject a
    #   fake; production uses the real Open3-based runner below.
    # binary: explicit binary path override (skips auto-resolution).
    def initialize(mode: BrowserMode::LIGHT, timeout: 30, runner: nil, binary: nil)
      @mode = mode
      @timeout = timeout
      @runner = runner || RealRunner.new
      @binary = binary || Binary.resolve!
    end

    # Binary path resolved at construction (parity with the other SDKs).
    def binary_path
      @binary
    end

    # Navigate + extract. wait_for: CSS selector (render mode).
    def goto(url, wait_for: nil)
      argv = [@binary, "goto", url, "--mode", @mode]
      argv.push("--wait-for", wait_for) if wait_for
      stdout, stderr, status = @runner.run(argv, timeout: @timeout)
      raise NavigationError, "b4n1web timed out after #{@timeout}s" if status.nil?
      unless status.zero?
        raise NavigationError, "b4n1web exited #{status}: #{stderr.to_s.strip}"
      end

      Page.parse(url, stdout)
    end

    # Execute JavaScript in the page context, return raw result string.
    def evaluate(url, js)
      argv = [@binary, "evaluate", url, js]
      stdout, stderr, status = @runner.run(argv, timeout: @timeout)
      raise NavigationError, "b4n1web timed out after #{@timeout}s" if status.nil?
      unless status.zero?
        raise NavigationError, "b4n1web exited #{status}: #{stderr.to_s.strip}"
      end

      stdout.to_s.strip
    end

    # The binary is stateless: close is a no-op kept for API parity.
    def close; end

    # Default runner: Open3 with hard timeout. On timeout the whole process
    # GROUP is killed (the binary spawns Chromium children that inherit
    # stdout; killing only the leader would leave them holding the pipe and
    # the readers below would block past the timeout) and status is nil
    # (so callers raise NavigationError). The group is also released after a
    # NORMAL exit so a lingering descendant can never hang the reads (parity
    # with the Go SDK's cmd.WaitDelay).
    class RealRunner
      def run(argv, timeout:)
        stdout = +""
        stderr = +""
        status = nil
        Open3.popen3(*argv, pgroup: true) do |pin, pout, perr, wait_thr|
          pid = wait_thr.pid
          reader = Thread.new { pout.read }
          err_reader = Thread.new { perr.read }
          begin
            Timeout.timeout(timeout) { status = wait_thr.value.exitstatus }
          rescue Timeout::Error
            status = nil
          end
          # Leader done (exit or timeout): kill the whole group so no
          # descendant keeps stdout/stderr open and blocks the drains.
          begin
            Process.kill("KILL", -pid) # negative pid = whole process group
          rescue Errno::ESRCH, Errno::EPERM
            Process.kill("KILL", pid) rescue nil
          end
          await_group_gone(pid)
          stdout = drain(reader)
          stderr = drain(err_reader)
          pin.close rescue nil
        end
        [stdout, stderr, status]
      end

      private

      # Join with a hard bound: after the group kill the pipes hit EOF
      # immediately; a pathological holder cannot hang the SDK.
      def drain(thread, seconds: 2)
        return thread.value if thread.join(seconds)
        thread.kill
        +""
      end

      # Signal delivery is asynchronous: after the KILL above the group is
      # signalled, not necessarily dead yet. Wait (bounded) until the group
      # is actually gone so callers get a deterministic "dead on return"
      # contract (parity with Go's cmd.WaitDelay). A killed orphan that is
      # still awaiting reaping answers kill(0, ...) — the bound keeps this
      # honest under any load.
      def await_group_gone(pgid, seconds: 2)
        deadline = Process.clock_gettime(Process::CLOCK_MONOTONIC) + seconds
        loop do
          begin
            Process.kill(0, -pgid)
          rescue Errno::ESRCH
            return
          end
          return if Process.clock_gettime(Process::CLOCK_MONOTONIC) >= deadline
          sleep 0.01
        end
      end
    end
  end
end
