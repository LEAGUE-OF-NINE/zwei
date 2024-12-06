import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
const CloneLimbus = () => {
  const [path, setPath] = useState<string | null>(null);

  async function selectFolder() {
    const file = await open({
      multiple: false,
      directory: true,
    });
    setPath(file);
  }
  
  async function cloneFolder() {
    if (path) {
      try {
        // Call the Tauri command to clone the folder
        await invoke("clone_folder_to_game", { srcPath: path });
        console.log("Folder cloned successfully to ./game");
      } catch (error) {
        console.error("Failed to clone folder:", error);
      }
    } else {
      console.error("No folder selected");
    }
  }

  return (
    <div className="flex items-center justify-center space-y-6 p-6">
      <button
        className="btn btn-primary flex items-center"
        onClick={selectFolder}
      >
        <span>Select Folder</span>
      </button>

      <div className="w-full flex text-center">
        <p>
          <strong>Selected Path:</strong>{" "}
          <span className="ont-mono">{path || "No folder selected yet."}</span>
        </p>
        {path && (
          <button
            className="btn btn-accent flex items-center space-x-2 px-4 py-2"
            onClick={cloneFolder}
          >
            <span>Clone</span>
          </button>
        )}
      </div>
    </div>
  );
};

export default CloneLimbus;
