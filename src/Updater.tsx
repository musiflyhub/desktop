import { useState, useEffect } from "react";
import { check } from "@tauri-apps/plugin-updater";
import { invoke } from "@tauri-apps/api/core";

export default function Updater() {
    const [status, setStatus] = useState("Checking for updates...");
    const [isUpdating, setIsUpdating] = useState(false);

    useEffect(() => {
        async function checkForUpdates() {
            try {
                const update = await check();
                if (update) {
                    console.log(`Update found: ${update.version}`);
                    setStatus(`Updating to ${update.version}...`);
                    setIsUpdating(true);

                    let downloaded = 0;
                    let contentLength = 0;

                    await update.downloadAndInstall((event) => {
                        switch (event.event) {
                            case 'Started':
                                contentLength = event.data.contentLength || 0;
                                console.log(`started downloading ${contentLength} bytes`);
                                break;
                            case 'Progress':
                                downloaded += event.data.chunkLength;
                                console.log(`downloaded ${downloaded} from ${contentLength}`);
                                break;
                            case 'Finished':
                                console.log('download finished');
                                break;
                        }
                    });

                    console.log('update installed');
                    await invoke("restart_app");
                } else {
                    console.log('no update found');
                    setStatus("Up to date!");
                    // Small delay before showing main window
                    setTimeout(async () => {
                        await invoke("show_main_window");
                    }, 1000);
                }
            } catch (error) {
                console.error("Update check failed:", error);
                setStatus("Update check failed. Starting app...");
                setTimeout(async () => {
                    await invoke("show_main_window");
                }, 1500);
            }
        }

        checkForUpdates();
    }, []);

    return (
        <div className="updater-container">
            <div className="spinner" style={{
                borderLeftColor: isUpdating ? '#4dff4d' : '#ff4d4d',
            }} />
            <h2 className="updater-title">Musifly</h2>
            <p className="updater-status">{status}</p>
        </div>
    );
}
