import { useState } from "react";
import { Outlet } from "react-router-dom";
import Sidebar from "./components/nav/Sidebar";
import TopbarMobile from "./components/nav/TopbarMobile";
import { useAppSelector } from "./state/hooks";
import Unauthorized from "./pages/public/errors/Unauthorized";

export default function AppLayout() {
  const user = useAppSelector((state) => state.user.user);
  const [sidebarOpen, setSidebarOpen] = useState<boolean>(false);

  if (!user) {
    return <Unauthorized />;
  } else {
    return (
      <div className="min-h-full">
        <Sidebar sidebarOpen={sidebarOpen} setSidebarOpen={setSidebarOpen} />
        <div className="lg:pl-64 print:!pl-0 flex flex-col">
          <TopbarMobile
            sidebarOpen={sidebarOpen}
            setSidebarOpen={setSidebarOpen}
          />
          <main className="flex-1">
            <Outlet />
          </main>
        </div>
      </div>
    );
  }
}
