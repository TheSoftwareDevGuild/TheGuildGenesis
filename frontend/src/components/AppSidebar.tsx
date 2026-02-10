import { Smile, BadgeCheck, Home, BookOpen , Table2, FolderKanban } from "lucide-react";

import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";

// Menu items.
const items = [
  {
    title: "Home",
    url: "/",
    icon: Home,
  },
  {
    title: "Getting Started",
    url: "/getting-started",
    icon: BookOpen,
  },
  {
    title: "Profiles",
    url: "/profiles",
    icon: Smile,
  },
  {
    title: "Badges",
    url: "/badges",
    icon: BadgeCheck,
  },
  {
    title: "Leaderboard",
    url: "/leaderboard",
    icon: Table2,
  },
   {
    title: "Projects",
    url: "/projects",
    icon: FolderKanban,
  },
];

export function AppSidebar() {
  return (
    <Sidebar>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Pages</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton asChild>
                    <a href={item.url}>
                      <item.icon />
                      <span>{item.title}</span>
                    </a>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
