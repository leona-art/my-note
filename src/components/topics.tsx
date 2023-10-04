import { createEffect, createSignal, onMount, For } from "solid-js";
import { invoke } from "@tauri-apps/api";

// #region Types
type Thing = {
    tb: string;
    id: Record<string, string>;
}
type TopicWithId = {
    title: string;
    content: string;
    id: Thing;
}

// #endregion

// #region States
const [dependTopics, loadTopics] = createSignal(undefined, { equals: false })
const [topics, setTopics] = createSignal<TopicWithId[]>([]);
createEffect(async () => {
    dependTopics()
    const response = await invoke("show_topic");
    console.log(response)
    setTopics(response as any[]);
})
// #endregion

// #region Components
export function TopicEditor() {
    const [title, setTitle] = createSignal("");
    const [content, setContent] = createSignal("");
    const handleCreate = async () => {
        await invoke("create_topic", { title: title(), content: content() });
        loadTopics()
    }
    return (
        <div>
            <input type="text" value={title()} onInput={({ target }) => setTitle(target.value)} />
            <input type="text" value={content()} onInput={({ target }) => setContent(target.value)} />
            <button onClick={handleCreate}>create</button>
        </div>
    )
}

export function TopicList() {
    onMount(() => {
        loadTopics()
    })
    return (
        <div>
            <div>topic list</div>
            <For each={topics()}>
                {(topic) => (
                    <p>{topic.title}</p>
                )}
            </For>
        </div>
    )
}

// #endregion