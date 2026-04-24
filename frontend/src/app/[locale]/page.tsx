import { Navbar } from "@/components/navbar/navbar";
import { TooltipProvider } from "@/components/ui/tooltip";

export default function HomePage() {
  return (
    <TooltipProvider>
      <Navbar />
    </TooltipProvider>
  );
}
