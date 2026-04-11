package b4n1web

import "fmt"

// BinaryNotFoundError is thrown when the b4n1web binary is not found
type BinaryNotFoundError struct {
	message string
}

func (e *BinaryNotFoundError) Error() string {
	return e.message
}

// NewBinaryNotFoundError creates a new BinaryNotFoundError
func NewBinaryNotFoundError() *BinaryNotFoundError {
	return &BinaryNotFoundError{
		message: "B4n1Web binary not found. Please install it first:\n  curl -sL https://web.b4n1.com/install | bash",
	}
}

// NavigationError is thrown when navigation fails
type NavigationError struct {
	url string
	err error
}

func (e *NavigationError) Error() string {
	return fmt.Sprintf("navigation to %s failed: %v", e.url, e.err)
}

func (e *NavigationError) Unwrap() error {
	return e.err
}

// NewNavigationError creates a new NavigationError
func NewNavigationError(url string, err error) *NavigationError {
	return &NavigationError{url: url, err: err}
}
