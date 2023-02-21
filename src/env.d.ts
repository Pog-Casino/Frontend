/// <reference types="astro/client" />
/// <reference types="@types/alpinejs" />

import type { Alpine } from "@types/alpinejs";

declare global {
  var turnstileCallback: (token: string) => void | undefined;
  var Alpine: Alpine | undefined;
}
