package com.b4n1.web;

/**
 * Options for AgentBrowser.
 */
public class BrowserOptions {
    private BrowserMode mode = BrowserMode.LIGHT;
    private int timeout = 30;
    private String userAgent = "B4N1Web-Agent/1.0";

    public BrowserOptions() {
    }

    public BrowserMode getMode() {
        return mode;
    }

    public void setMode(BrowserMode mode) {
        this.mode = mode;
    }

    public int getTimeout() {
        return timeout;
    }

    public void setTimeout(int timeout) {
        this.timeout = timeout;
    }

    public String getUserAgent() {
        return userAgent;
    }

    public void setUserAgent(String userAgent) {
        this.userAgent = userAgent;
    }
}
