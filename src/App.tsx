import "./App.css";
import { TopicEditor, TopicList } from "./components/topics";

function App() {
  return (
    <div class="container">
      <TopicEditor />
      <TopicList />
    </div>
  );
}

export default App;
