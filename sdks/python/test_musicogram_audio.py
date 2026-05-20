import sys
from b4n1web.browser import AgentBrowser, BrowserMode

def main():
    print("Initializing B4N1-WEB SDK in Render mode...")
    browser = AgentBrowser(mode=BrowserMode.RENDER)
    url = "https://dev.musicogram.com/en/post/harmonics/"
    
    print(f"Navigating and Evaluating background audio state via CDP JS execution...")
    # This evaluates JavaScript inside the Chromium page context
    js_code = """
        () => {
            const playerState = {
                narrationReady: window.playerState?.narrationReady || false,
                musicReady: window.playerState?.musicReady || false,
                currentTime: window.audio?.currentTime || 0,
                musicTime: window.music?.currentTime || 0,
                musicVolume: window.music?.volume || 0,
                musicPaused: window.music?.paused,
                audioPaused: window.audio?.paused
            };
            return JSON.stringify(playerState);
        }
    """
    
    try:
        result = browser.evaluate(url, js_code)
        print("Evaluation Result:")
        print(result)
        
        # Check if the player exists and music is loaded
        if "musicReady" in result and "true" in result:
            print("SUCCESS: Music player state accessible via evaluate!")
        else:
            print("WARNING: Player state might not be fully initialized or playing yet.")
    except Exception as e:
        print(f"Failed to evaluate JS: {e}")

if __name__ == "__main__":
    main()
