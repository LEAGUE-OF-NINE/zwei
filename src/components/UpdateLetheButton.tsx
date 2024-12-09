import { invoke } from "@tauri-apps/api/core";
import { useErrorHandler } from "../context/useErrorHandler";

const UpdateLetheButton = () => {
  const handleError = useErrorHandler();
  async function updateLethe() {
    try {
      await invoke("download_and_install_lethe");
    } catch (error) {
      handleError(error);
      console.error("Failed to clone folder:", error);
    }
  }
  return <button className="btn btn-sm btn-ghost" onClick={updateLethe}>Update Lethe</button>;
};

export default UpdateLetheButton;
