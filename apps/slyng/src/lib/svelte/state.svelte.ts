import { page } from "$app/state";

export const currentPath: { value: string } = $state({
    value: page.url.pathname,
});
