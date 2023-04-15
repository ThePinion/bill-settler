import MenuElement from "./MenuElement";
import ProfileComponent, { Profile } from "./Profile";

export default function SideBar() {
    return(
        <div
            class="w-40 pt-6 flex flex-col
            bg-gray-900 text-white shadow-lg"
        >
            <ProfileComponent profile={new Profile("User123")}></ProfileComponent>
            <MenuElement text="Home"></MenuElement>
            <MenuElement text="Expenses"></MenuElement>
            <MenuElement text="Groups"></MenuElement>
            <MenuElement text="Friends"></MenuElement>
            <MenuElement text="Chats"></MenuElement>
        </div>
    );
}