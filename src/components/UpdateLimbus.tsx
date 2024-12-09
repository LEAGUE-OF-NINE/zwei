import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import Modal from "./Modal";
import { useErrorHandler } from "../context/useErrorHandler";

const UpdateLimbus = () => {
  const [path, setPath] = useState<string | null>(null);
  const [isModalOpen, setModalOpen] = useState(false);
  const [info, setInfo] = useState<string | null>(null);
  const handleError = useErrorHandler();
  
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
        setInfo("Cloning game folder...");
        await invoke("clone_folder_to_game", { srcPath: path });
        setInfo("Downloading and extracting bepinex...");
        await invoke("download_and_extract_bepinex");
        setInfo("Downloading and extracting lethe...");
        await invoke("download_and_install_lethe");
        setInfo("Patching limbus...");
        await invoke("patch_limbus", { srcPath: path });
      } catch (error) {
        console.error("Failed to update limbus:", error);
        handleError(error);
      } finally {
        setInfo(null);
        setModalOpen(false);
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

      <Modal
        isOpen={isModalOpen}
        title="Update/Install Limbus"
        onClose={() => setModalOpen(false)}
        actions={
          <>
            <button
              className="btn btn-primary"
              onClick={updateLimbus}
              disabled={info != null}
            >
              Update Limbus
            </button>
          </>
        }
      >
        <input
          type="text"
          className="input w-full mb-4 bg-base-200 hover:cursor-pointer border-disabled"
          onClick={selectFolder}
          placeholder={path || "Select Limbus Folder..."}
          readOnly
        />
        {info && <p>{info}</p>}
      </Modal>
    </div>
  );
};

export default UpdateLimbus;
