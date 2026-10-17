//go:build !windows

package b4n1web

import (
	"os/exec"
	"syscall"
)

// prepareProcess puts the child in its own process group so a timeout kills
// the whole tree (b4n1web spawns Chromium children - killing only the leader
// would orphan them).
func prepareProcess(cmd *exec.Cmd) {
	cmd.SysProcAttr = &syscall.SysProcAttr{Setpgid: true}
}

// killProcessGroup SIGKILLs the child's whole process group. Safe to call
// after the leader already exited: grandchildren (e.g. a renderer holding
// stdout) are the ones we need gone.
func killProcessGroup(cmd *exec.Cmd) {
	if cmd.Process != nil && cmd.Process.Pid > 0 {
		_ = syscall.Kill(-cmd.Process.Pid, syscall.SIGKILL)
	}
}
