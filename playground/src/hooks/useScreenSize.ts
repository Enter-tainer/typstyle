import { useEffect, useState } from "react";
import type { ScreenSizeType } from "../types";

export function useScreenSize(): ScreenSizeType {
  const [screenSize, setScreenSize] = useState<ScreenSizeType>(() => {
    const width = window.innerWidth;
    if (width >= 1200) return "wide";
    if (width >= 768) return "medium";
    return "thin";
  });

  useEffect(() => {
    const updateScreenSize = () => {
      const width = window.innerWidth;
      if (width >= 1200) setScreenSize("wide");
      else if (width >= 768) setScreenSize("medium");
      else setScreenSize("thin");
    };

    updateScreenSize(); // initial check
    window.addEventListener("resize", updateScreenSize);

    return () => window.removeEventListener("resize", updateScreenSize);
  }, []);

  return screenSize;
}
