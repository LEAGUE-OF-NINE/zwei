import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
const UpdateLimbus = () => {
  const [path, setPath] = useState<string | null>(null);
  const [isModalOpen, setModalOpen] = useState(false);

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
        console.error("Failed to update limbus:", error);
      }
    } else {
      console.error("No folder selected");
    }
  }

  return (
    <div>
      <button
        className="btn btn-sm btn-ghost"
        onClick={() => setModalOpen(true)}
      >
        Update/Install Limbus
      </button>

      {isModalOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
          <div className="modal-box">
            <h2 className="text-lg font-bold mb-4">Update/Install Limbus</h2>
            <input
              type="text"
              className="input w-full mb-4 bg-base-200 hover:cursor-pointer border-disabled"
              onClick={selectFolder}
              placeholder={path || "Select Limbus Folder..."}
              readOnly
            />
            <div className="flex justify-end space-x-2">
              <button
                className="btn btn-secondary"
                onClick={() => setModalOpen(false)}
              >
                Cancel
              </button>
              <button
                className="btn btn-primary"
                onClick={() => {
                  updateLimbus();
                  setModalOpen(false);
                }}
              >
                Update Limbus
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default UpdateLimbus;
