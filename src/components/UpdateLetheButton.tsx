import { invoke } from "@tauri-apps/api/core";
import { useErrorHandler } from "../context/useErrorHandler";
import { useToast } from "../context/ToastContext";
import { useState } from "react";

const UpdateLetheButton = () => {
  const handleError = useErrorHandler();
  const { showToast } = useToast();
  const [updating, setUpdating] = useState(false);
  async function updateLethe() {
    try {
      setUpdating(true);
      await invoke("download_and_install_lethe");
      showToast("Lethe updated successfully", 'alert-success');
    } catch (error) {
      handleError(error);
      console.error("Failed to clone folder:", error);
    } finally {
      setUpdating(false);
    }
  }
  return (
    <>
      <button
        className="btn btn-sm btn-ghost"
        onClick={updateLethe}
        disabled={updating}
      >
        Update Lethe
      </button>
    </>
  );
};

export default UpdateLetheButton;
