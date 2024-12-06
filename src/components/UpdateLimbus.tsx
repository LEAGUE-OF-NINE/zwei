import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
const UpdateLimbus = () => {
  const [path, setPath] = useState<string | null>(null);

  async function selectFolder() {
    const file = await open({
      multiple: false,
      directory: true,
    });
    setPath(file);
  }

  async function updateLimbus() {
    if (path) {
      try {
        // Call the Tauri command to clone the folder
        await invoke("clone_folder_to_game", { srcPath: path });
        await invoke("download_and_extract_bepinex");
        await invoke("download_and_install_lethe");
      } catch (error) {
        console.error("Failed to clone folder:", error);
      }
    } else {
      console.error("No folder selected");
    }
  }

  return (
    <div>
      <input
        type="text"
        className="input max-w-xs hover:cursor-pointer border-disabled"
        onClick={selectFolder}
        placeholder={path || "Select Folder..."}
        readOnly
      />
      <button className="btn btn-accent" onClick={updateLimbus} disabled={!path}>
        <span>Update Limbus</span>
      </button>
    </div>
  );
};

export default UpdateLimbus;
