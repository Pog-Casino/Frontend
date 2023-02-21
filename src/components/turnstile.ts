export function withTurnstile(callback: (token: string) => void) {
  const token = Alpine!.store("turnstile-token");
  if (token) {
    callback(token as string);
  } else {
    window.addEventListener("turnstile:ready", () => {
      withTurnstile(callback);
    }, {
      once: true,
    });
  }
}

export function setTurnstile(token: string) {
  Alpine!.store("turnstile-token", token);
  window.dispatchEvent(new Event("turnstile:ready"));
}
