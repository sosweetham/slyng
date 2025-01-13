import { Store } from "@tauri-apps/plugin-store";
import { supportedThemeNames } from "../global/themes.ts";

/**
 * A singleton to manage savable config of the app
 *
 * @property {string} theme - The current theme of the app
 */
export class Settings {
    private _store: Store;
    private _theme: string = $state("light");

    private constructor(store: Store) {
        this._store = store;
    }

    static async create() {
        const store = await Store.load("slyng-settings.json", {
            autoSave: true,
        });
        const instance = new Settings(store);
        await instance.init();
        return instance;
    }

    // init function to load promised data
    async init() {
        const presentTheme = await this._store.get<string>("theme");
        console.log("Present theme", presentTheme);
        if (!presentTheme) {
            console.log("Setting theme to light");
            await this._store.set("theme", "light");
            this._theme = "light";
            return;
        }
        this._theme = presentTheme;
    }

    get theme() {
        return this._theme;
    }

    set theme(value: string) {
        if (!supportedThemeNames.includes(value)) {
            throw new Error(`Unsupported theme: ${value}`);
        }
        console.log("Setting theme to", value);
        this._theme = value;
        this._store.set("theme", value);
    }
}
