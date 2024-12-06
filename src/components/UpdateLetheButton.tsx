import { invoke } from "@tauri-apps/api/core";

const UpdateLetheButton = () => {
  async function updateLethe() {
    try {
      await invoke("download_and_install_lethe");
    } catch (error) {
      console.error("Failed to clone folder:", error);
    }
  }
  return <button className="btn btn-primary" onClick={updateLethe}>Update Lethe</button>;
};

export default UpdateLetheButton;
