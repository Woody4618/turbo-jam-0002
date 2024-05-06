import initTurbo, * as turbo from "./pkg/turbo_genesis_host_wasm_bindgen.js";

/**************************************************/
/* CONFIGURATION                                  */
/**************************************************/

const APP_NAME = "Angry Bonk";
const APP_VERSION = "1.0";
const APP_AUTHOR = "Jonas und Eike";
const APP_DESCRIPTION = "Defend the sheep from the angry bonk";
const RESOLUTION = [800, 600];
const WASM_SRC = "angry_bonk.wasm";

const SPRITES = [
  "./sprites/dog.png",
  "./sprites/cloud-5.png",
  "./sprites/wood-hbeam-1.png",
  "./sprites/dead_sheep.png",
  "./sprites/cloud-4.png",
  "./sprites/create-big.png",
  "./sprites/cloud-6.png",
  "./sprites/sheep_closed.png",
  "./sprites/cloud-7.png",
  "./sprites/bonk.png",
  "./sprites/ball.png",
  "./sprites/cloud-3.png",
  "./sprites/cloud-2.png",
  "./sprites/cloud-1.png",
  "./sprites/man-front.png",
  "./sprites/sheep_animation.gif",
  "./sprites/cannon_wheel.png",
  "./sprites/crate-metal.png",
  "./sprites/wood-dbeam-1.png",
  "./sprites/sheep.png",
  "./sprites/explosion_sheet.png",
  "./sprites/cannon_barrel.png",
  "./sprites/grass_patch.png",
  "./sprites/cannon_flame2.png",
  "./sprites/man-back.png",
  "./sprites/angry.png",
  "./sprites/cannon_flame.png",
  "./sprites/crate-small.png",
  "./sprites/logo.png",
  "./sprites/wood-beam-2.png",
  "./sprites/wood-beam-1.png",
  "./sprites/logo_angry.png",
  "./sprites/sheep_animation.png",
  "./sprites/log.png",
  "./sprites/bonk-dog.png",
  "./sprites/crate-small-dmg1.png",
  "./sprites/barrel.png",
  "./sprites/cloud-8.png",
];

/**************************************************/

// This proxy prevents WebAssembly.LinkingError from being thrown
// prettier-ignore
window.createWasmImportsProxy = (target = {}) => {
  console.log(target);
  return new Proxy(target, {
    get: (target, namespace) => {
      // Stub each undefined namespace with a Proxy
      target[namespace] = target[namespace] ?? new Proxy({}, {
        get: (_, prop) => {
          // Generate a sub function for any accessed property
          return (...args) => {
            console.log(`Calling ${namespace}.${prop} with arguments:`, args);
            // Implement the actual function logic here
          };
        }
      });
      return target[namespace];
    }
  })
};

window.turboSolUser = window.turboSolUser ?? (() => null);
window.turboSolGetAccount = window.turboSolGetAccount ?? (async () => {});
window.turboSolSignAndSendTransaction =
  window.turboSolSignAndSendTransaction ?? (async () => {});

/**************************************************/

try {
  // Initalize Turbo's WASM runtime
  await initTurbo();

  // Create the game's canvas
  const player = document.getElementById("player");

  // Initialize a temporary 2D context canvas for loading state
  const loading = document.createElement("canvas");
  player?.appendChild(loading);
  var context = loading.getContext("2d");
  context.fillStyle = "white";
  context.font = "bold 14px 04b03";
  context.textAlign = "center";
  context.textBaseline = "middle";
  context.fillText("Loading...", loading.width / 2, loading.height / 2);

  // Fetch sprites
  const spriteData = await Promise.all(
    SPRITES.map(async (src) => {
      try {
        let res = await fetch(src);
        let buf = await res.arrayBuffer();
        return [
          src.replace(/^.*[\\/]/, "").replace(/.(png|jpg|jpeg|gif)$/, ""),
          buf,
        ];
      } catch (err) {
        console.error("Could not fetch sprite:", src);
        return null;
      }
    }).filter((x) => !!x)
  );

  // Remove loading state
  player?.removeChild(loading);

  // Append game canvas
  const canvas = document.createElement("canvas");
  player?.appendChild(canvas);

  // Initialize nipple (aka virtual analog stick)
  initializeNipple(canvas);

  // Run game
  await turbo.run(canvas, spriteData, {
    source: WASM_SRC,
    meta: {
      appName: APP_NAME,
      appVersion: APP_VERSION,
      appAuthor: APP_AUTHOR,
      appDescription: APP_DESCRIPTION,
    },
    config: {
      resolution: RESOLUTION,
    },
  });
} catch (err) {
  console.error("Turbo failed to initialize", err);
}

function initializeNipple(canvas) {
  const presses = {
    up: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowUp",
        code: "ArrowUp",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowUp",
        code: "ArrowUp",
      }),
    },
    down: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowDown",
        code: "ArrowDown",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowDown",
        code: "ArrowDown",
      }),
    },
    left: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowLeft",
        code: "ArrowLeft",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowLeft",
        code: "ArrowLeft",
      }),
    },
    right: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowRight",
        code: "ArrowRight",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowRight",
        code: "ArrowRight",
      }),
    },
  };
  let active = null;
  nipplejs
    .create()
    .on("dir:up", (e) => {
      console.log(e);
      if (active && active !== presses.up) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.up.keydown);
      active = presses.up;
    })
    .on("dir:down", (e) => {
      console.log(e);
      if (active && active !== presses.down) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.down.keydown);
      active = presses.down;
    })
    .on("dir:left", (e) => {
      console.log(e);
      if (active && active !== presses.left) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.left.keydown);
      active = presses.left;
    })
    .on("dir:right", (e) => {
      console.log(e);
      if (active && active !== presses.right) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.right.keydown);
      active = presses.right;
    })
    .on("end", (e) => {
      console.log(e);
      if (active) {
        canvas.dispatchEvent(active.keyup);
      }
      active = null;
    });
}
