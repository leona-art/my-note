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

/** トピックの編集 */
export function TopicEditor() {
    const [title, setTitle] = createSignal("");
    const [content, setContent] = createSignal("");
    const handleCreate = async () => {
        await invoke("create_topic", { title: title(), content: content() });
        loadTopics()
    }
    return (
        <div>
            <input
                type="text"
                value={title()}
                placeholder="タイトルを入力してください"
                onInput={({ target }) => setTitle(target.value)}
                style={{
                    "display": "block",
                    "margin-left": "auto",
                    "margin-right": "auto",
                }}
            />

            <textarea
                value={content()}
                onInput={({ target }) => setContent(target.value)}
                placeholder="内容を入力してください"
                cols={30}
                rows={10}
                style={{
                    "display": "block",
                    "font-size": "1.2rem",
                    "width": "100%"
                }}
            />
            <button onClick={handleCreate}>create</button>
        </div>
    )
}

/** トピックの一覧表示 */
export function TopicList() {
    onMount(() => {
        loadTopics()
    })
    return (
        <div>
            <ul>
                <For each={topics()}>
                    {(topic) => (
                        <li>
                            <p>
                                <span>{topic.title}</span>
                                <span>{topic.content}</span>
                            </p>
                        </li>
                    )}
                </For>
            </ul>

        </div>
    )
}

// #endregion