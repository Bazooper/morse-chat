const { invoke } = window.__TAURI__.core;

const greetButton = document.querySelector("#greet-button");
const greetInput = document.querySelector("#greet-input");

async function greet(name) {
  return await invoke("greet", {name: name});
}

greet("").then((message) => {
  document.querySelector("#message").innerText = message;
});

greetButton.addEventListener("click", () => {
  greet(greetInput.value).then((message) => {
    document.querySelector("#message").innerText = message;
  });
});
