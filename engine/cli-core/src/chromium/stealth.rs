//! Stealth module to bypass bot detection

use crate::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn apply_stealth(page: &chromiumoxide::Page) -> Result<()> {
    let js = format!(r#"
    (function() {{
        // === STEALTH: Ocultar automation ===
        Object.defineProperty(navigator, 'webdriver', {{ get: () => false }});
        if (window.chrome) {{
            Object.defineProperty(window.chrome, 'runtime', {{ get: () => undefined }});
        }}
        const originalQuery = window.navigator.permissions?.query;
        if (originalQuery) {{
            window.navigator.permissions.query = (p) => {{
                if (p.name === 'notifications') return Promise.resolve({{ state: 'denied' }});
                return originalQuery(p);
            }};
        }}
        Object.defineProperty(navigator, 'plugins', {{ get: () => [1, 2, 3, 4, 5] }});
        Object.defineProperty(navigator, 'languages', {{ get: () => ['en-US', 'en'] }});
        ['webdriver','__webdriver_eval','__selenium_evaluate',
         '__selenium_unwrapped','__driver_evaluate','__fxdriver_evaluate',
         '__webdriver_script_fn'].forEach(k => {{ try {{ delete window[k]; }} catch(e){{}} }});
        if (window.navigator.connection) {{
            Object.defineProperty(navigator.connection, 'rtt', {{ get: () => 100 }});
        }}

        // === FINGERPRINT RANDOMIZATION ===
        const cpus = [2, 4, 6, 8, 12, 16];
        Object.defineProperty(navigator, 'hardwareConcurrency', {{
            get: () => cpus[{}]
        }});

        const mems = [0.25, 0.5, 1, 2, 4, 8];
        Object.defineProperty(navigator, 'deviceMemory', {{
            get: () => mems[{}]
        }});

        const platforms = ['Win32', 'MacIntel', 'Linux x86_64', 'Linux aarch64'];
        Object.defineProperty(navigator, 'platform', {{
            get: () => platforms[{}]
        }});

        const getExt = WebGLRenderingContext.prototype.getExtension;
        WebGLRenderingContext.prototype.getExtension = function() {{
            const result = getExt.apply(this, arguments);
            if (arguments[0] === 'WEBGL_debug_renderer_info') return null;
            return result;
        }};

        const origToDataURL = HTMLCanvasElement.prototype.toDataURL;
        HTMLCanvasElement.prototype.toDataURL = function() {{
            const data = origToDataURL.apply(this, arguments);
            const ctx = this.getContext('2d');
            if (ctx) {{
                ctx.fillStyle = `rgba(${{{} % 255}}, ${{{} % 255}}, ${{{} % 255}}, 0.01)`;
                ctx.fillRect(0, 0, 1, 1);
            }}
            return data;
        }};
    }})();
    "#,
    rand_range(0, 5),
    rand_range(0, 5),
    rand_range(0, 3),
    rand_range(0, 255), rand_range(0, 255), rand_range(0, 255)
);

    page.evaluate(js)
        .await
        .map_err(|e| crate::Error::Other(format!("Stealth error: {}", e)))?;

    Ok(())
}

fn rand_range(min: usize, max: usize) -> usize {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    min + (nanos as usize) % (max - min + 1)
}
