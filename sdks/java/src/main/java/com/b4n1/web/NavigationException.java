package com.b4n1.web;

/**
 * Exception thrown when navigation fails.
 */
public class NavigationException extends RuntimeException {
    private final String url;

    public NavigationException(String url, String message) {
        super(message);
        this.url = url;
    }

    public NavigationException(String url, Throwable cause) {
        super(cause);
        this.url = url;
    }

    public String getUrl() {
        return url;
    }
}
