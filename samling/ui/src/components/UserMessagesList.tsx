import { useAppSelector } from "../state/hooks";
import UserMessage from "./UserMessage";

export default function UserMessagesList() {
  const messages = useAppSelector((state) => state.user.messages);
  return (
    <div id="messages" className="fixed inset-0 pointer-events-none z-50">
      {messages
        .filter((message) => message.dismissed === false)
        .map((message) => (
          <UserMessage key={message.id} message={message} />
        ))}
    </div>
  );
}
