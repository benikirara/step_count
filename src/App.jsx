import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

function App() {
  const [sourcePath, setSourcePath] = useState("");
  const [startRevision, setStartRevision] = useState("");
  const [endRevision, setEndRevision] = useState("");
  const [userName, setUserName] = useState("");

  const execute = async () => {
    if (!sourcePath) return;
    if (!startRevision) return;
    if (!endRevision) return;
    if (!userName) return;

    const userRequestData = {
      sourcePath,
      startRevision,
      endRevision,
      userName,
    };

    try {
      const stepCountResultData = await invoke('exec', { userRequestData });
      console.log(stepCountResultData);
    } catch (e) {
      alert(e);
    }
  }

  return (
    <>
      ソースコードパス <input value={sourcePath} onChange={e => setSourcePath(e.target.value)} /><br />
      開始リビジョン <input value={startRevision} onChange={e => setStartRevision(e.target.value)} /><br />
      終了リビジョン <input value={endRevision} onChange={e => setEndRevision(e.target.value)} /><br />
      ユーザー名 <input value={userName} onChange={e => setUserName(e.target.value)} /><br />
      <button onClick={execute}>実行</button>
    </>
  );
}

export default App;
