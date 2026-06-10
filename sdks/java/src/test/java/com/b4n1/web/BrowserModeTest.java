package com.b4n1.web;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Smoke tests for the {@link BrowserMode} enum.
 *
 * Verifies enum constants, their string representations,
 * and the reverse mapping from string to enum value.
 */
class BrowserModeTest {

    // ── Enum constants ────────────────────────────────────────────────

    @Test
    void enumShouldContainThreeConstants() {
        BrowserMode[] values = BrowserMode.values();
        assertEquals(3, values.length, "There should be exactly three browser modes");
    }

    @Test
    void enumShouldContainLight() {
        assertNotNull(BrowserMode.valueOf("LIGHT"), "LIGHT constant must exist");
    }

    @Test
    void enumShouldContainJs() {
        assertNotNull(BrowserMode.valueOf("JS"), "JS constant must exist");
    }

    @Test
    void enumShouldContainRender() {
        assertNotNull(BrowserMode.valueOf("RENDER"), "RENDER constant must exist");
    }

    // ── String representation (getValue) ──────────────────────────────

    @Test
    void lightShouldHaveValueLight() {
        assertEquals("light", BrowserMode.LIGHT.getValue(),
                "LIGHT mode should map to \"light\"");
    }

    @Test
    void jsShouldHaveValueJs() {
        assertEquals("js", BrowserMode.JS.getValue(),
                "JS mode should map to \"js\"");
    }

    @Test
    void renderShouldHaveValueRender() {
        assertEquals("render", BrowserMode.RENDER.getValue(),
                "RENDER mode should map to \"render\"");
    }

    // ── Reverse mapping (fromString) ──────────────────────────────────

    @Test
    void fromStringShouldReturnLight() {
        assertEquals(BrowserMode.LIGHT, fromString("light"));
    }

    @Test
    void fromStringShouldReturnJs() {
        assertEquals(BrowserMode.JS, fromString("js"));
    }

    @Test
    void fromStringShouldReturnRender() {
        assertEquals(BrowserMode.RENDER, fromString("render"));
    }

    @Test
    void fromStringShouldBeCaseSensitive() {
        assertThrows(IllegalArgumentException.class, () -> fromString("LIGHT"));
        assertThrows(IllegalArgumentException.class, () -> fromString("Light"));
    }

    @Test
    void fromStringShouldThrowForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> fromString("headless"));
        assertThrows(IllegalArgumentException.class, () -> fromString(""));
        assertThrows(IllegalArgumentException.class, () -> fromString(null));
    }

    // ── Ordinals ──────────────────────────────────────────────────────

    @Test
    void lightShouldBeFirstConstant() {
        assertEquals(0, BrowserMode.LIGHT.ordinal());
    }

    @Test
    void jsShouldBeSecondConstant() {
        assertEquals(1, BrowserMode.JS.ordinal());
    }

    @Test
    void renderShouldBeThirdConstant() {
        assertEquals(2, BrowserMode.RENDER.ordinal());
    }

    // ── Helper: reverse string-to-enum mapping ────────────────────────
    // NOTE: BrowserMode currently lacks a built-in fromString() method.
    // This helper demonstrates the expected contract. If the SDK adds a
    // fromString() method later, tests should migrate to it.

    static BrowserMode fromString(String value) {
        if (value == null) {
            throw new IllegalArgumentException("BrowserMode value must not be null");
        }
        for (BrowserMode mode : BrowserMode.values()) {
            if (mode.getValue().equals(value)) {
                return mode;
            }
        }
        throw new IllegalArgumentException("Unknown BrowserMode value: " + value);
    }
}
