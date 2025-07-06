import React, { useState, useEffect } from "react";

function App() {
  const [tasks, setTasks] = useState([]);
  const [desc, setDesc] = useState("");
  const [priority, setPriority] = useState(3);

  const apiUrl = "http://localhost:8080";

  useEffect(() => {
    fetch(`${apiUrl}/tasks`)
      .then(res => res.json())
      .then(setTasks);
  }, []);

  const addTask = () => {
    fetch(`${apiUrl}/tasks`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ description: desc, priority: parseInt(priority) }),
    })
      .then(res => res.json())
      .then(task => {
        setTasks([...tasks, task]);
        setDesc("");
        setPriority(3);
      });
  };

  const toggleDone = (id) => {
    fetch(`${apiUrl}/tasks/${id}/toggle`, { method: "POST" }).then(() => {
      setTasks(tasks.map(t => (t.id === id ? { ...t, done: !t.done } : t)));
    });
  };

  const removeTask = (id) => {
    fetch(`${apiUrl}/tasks/${id}`, { method: "DELETE" }).then(() => {
      setTasks(tasks.filter(t => t.id !== id));
    });
  };

  return (
    <div style={{ maxWidth: 600, margin: "auto", padding: 20 }}>
      <h1>To-Do List</h1>

      <ul>
        {tasks.map(task => (
          <li key={task.id}>
            <input type="checkbox" checked={task.done} onChange={() => toggleDone(task.id)} />
            {task.description} (Priority: {task.priority})
            <button onClick={() => removeTask(task.id)}>Delete</button>
          </li>
        ))}
      </ul>

      <input
        placeholder="Task description"
        value={desc}
        onChange={e => setDesc(e.target.value)}
      />
      <select value={priority} onChange={e => setPriority(e.target.value)}>
        {[1,2,3,4,5].map(n => (
          <option key={n} value={n}>{n}</option>
        ))}
      </select>
      <button onClick={addTask}>Add Task</button>
    </div>
  );
}

export default App;
