//go:build windows

package b4n1web

import "os/exec"

// prepareProcess: no process groups on Windows; exec.CommandContext already
// kills the direct child on timeout.
func prepareProcess(cmd *exec.Cmd) {}

// killProcessGroup: best-effort fallback - kill the direct child only.
func killProcessGroup(cmd *exec.Cmd) {
	if cmd.Process != nil {
		_ = cmd.Process.Kill()
	}
}
