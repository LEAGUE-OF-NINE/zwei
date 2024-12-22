import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import Modal from "./Modal";
import { useErrorHandler } from "../context/useErrorHandler";
import { useToast } from "../context/ToastContext";

const UpdateLimbus = () => {
  const [path, setPath] = useState<string | null>(null);
  const [isModalOpen, setModalOpen] = useState(false);
  const [info, setInfo] = useState<string | null>(null);
  const handleError = useErrorHandler();
  const { showToast } = useToast();

  async function selectFolder() {
    const response = await invoke<string>("steam_limbus_location");
    const file = await open({
      multiple: false,
      directory: true,
      defaultPath: response == "" ? undefined : response,
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
        showToast("Limbus Updated Successfully", "alert-success");
      } catch (error) {
        console.error("Failed to update limbus:", error);
        handleError(error);
      } finally {
        setInfo(null);
        setModalOpen(false);
      }
    } else {
      showToast("No folder selected", "alert-error");
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
        <label className="form-control w-full">
          <div className="label">
            <span className="label-text">
              Select your original Limbus Company folder
            </span>
          </div>
          <input
            type="text"
            className="input w-full mb-4 bg-base-200 hover:cursor-pointer border-disabled"
            onClick={selectFolder}
            placeholder={path || "Select Limbus Folder..."}
            readOnly
          />
          <div className="label">
            <span className="label-text-alt">Zwei will make a copy of it and install Lethe on it</span>
          </div>
        </label>
        {info && <p>{info}</p>}
      </Modal>
    </div>
  );
};

export default UpdateLimbus;
