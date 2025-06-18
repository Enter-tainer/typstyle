import type React from "react";
import type { ScreenSizeType } from "@/types";
import { Panel, Tab, Tabs } from "./ui";

interface MainLayoutProps {
  screenSize: ScreenSizeType;
  optionsPanel: React.ReactNode;
  sourcePanel: React.ReactNode;
  formattedPanel: React.ReactNode;
  astPanel: React.ReactNode;
  irPanel: React.ReactNode;
}

export function MainLayout({
  screenSize,
  optionsPanel,
  sourcePanel,
  formattedPanel,
  astPanel,
  irPanel,
}: MainLayoutProps) {
  return (
    <div className="flex overflow-hidden min-h-0 h-full p-3 gap-2">
      {/* Wide Layout: 3 Columns */}
      {screenSize === "wide" && (
        <>
          <Panel header="Settings" className="flex-none w-[280px]">
            {optionsPanel}
          </Panel>
          <Panel header="Source" className="flex-1 min-w-0">
            {sourcePanel}
          </Panel>
          <Tabs defaultActiveTab="formatted" className="flex-1 min-w-0">
            <Tab tid="formatted" label="Formatted">
              {formattedPanel}
            </Tab>
            <Tab tid="ast" label="AST">
              {astPanel}
            </Tab>
            <Tab tid="ir" label="Pretty IR">
              {irPanel}
            </Tab>
          </Tabs>
        </>
      )}

      {/* Thin Layout: 1 Column (Full Width) */}
      {screenSize === "thin" && (
        <Tabs defaultActiveTab="source" className="flex-1 min-w-0">
          <Tab tid="options" label="Settings">
            {optionsPanel}
          </Tab>
          <Tab tid="source" label="Source">
            {sourcePanel}
          </Tab>
          <Tab tid="formatted" label="Formatted">
            {formattedPanel}
          </Tab>
          <Tab tid="ast" label="AST">
            {astPanel}
          </Tab>
          <Tab tid="ir" label="Pretty IR">
            {irPanel}
          </Tab>
        </Tabs>
      )}
    </div>
  );
}
