import { useState, useEffect } from "react";
import axios from "axios";

function App() {
  const [imageSrc, setImageSrc] = useState("");
  const [isLoading, setIsLoading] = useState(true);
  const backendUrl = import.meta.env.BACKEND_URL || "";

  useEffect(() => {
    const loadImage = async () => {
      try {
        setIsLoading(true);
        const url = backendUrl ? `${backendUrl}/picsum` : "/picsum";
        const response = await axios.get(url, { responseType: "blob" });
        const imageUrl = URL.createObjectURL(response.data);
        setImageSrc(imageUrl);
      } catch (error) {
        console.error("Error fetching image:", error);
      } finally {
        setIsLoading(false);
      }
    };

    loadImage();
  }, [backendUrl]);

  return (
    <>
      <h1>Devops With Kubernetes</h1>
      {isLoading ? (
        <p>Loading image...</p>
      ) : (
        <img src={imageSrc} alt="a random image" />
      )}
    </>
  );
}

export default App;
