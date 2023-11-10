import { useState, useEffect } from "react";
import axios from "axios";

const todos = [
  {
    id: 1,
    title: "Do laundry",
    completed: false,
  },
  {
    id: 2,
    title: "Cook dinner",
    completed: true,
  },
];

function App() {
  const [imageSrc, setImageSrc] = useState("");
  const [isLoading, setIsLoading] = useState(true);
  const is_dev_mode = import.meta.env.MODE === "development";
  const backendUrl = is_dev_mode ? "http://localhost:3000" : "";

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
      <div>
        <input maxLength={140} /> <button>Create Todo</button>
      </div>
      <ul>
        {todos.map((todo) => (
          <li key={todo.id}>
            <input type="checkbox" checked={todo.completed} />
            {todo.title}
          </li>
        ))}
      </ul>
    </>
  );
}

export default App;
