// App.jsx
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Mainpage from "./frontend/main/Mainpage"
import Projectpage from "./components/Projectpage"

function App() {
  return (
    <BrowserRouter>
      <Routes>
            <Route path="/" element={<Mainpage />} />
            <Route path="/projects" element={<Projectpage />}>
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;