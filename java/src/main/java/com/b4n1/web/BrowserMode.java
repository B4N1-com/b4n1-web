package com.b4n1.web;

/**
 * Browser execution modes.
 */
public enum BrowserMode {
    LIGHT("light"),
    JS("js"),
    RENDER("render");

    private final String value;

    BrowserMode(String value) {
        this.value = value;
    }

    public String getValue() {
        return value;
    }
}
