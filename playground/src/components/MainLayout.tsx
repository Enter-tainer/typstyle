import type { ScreenSizeType } from "@/types";
import type React from "react";
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
    <div className="flex overflow-hidden min-h-0 h-full p-4 gap-2">
      {screenSize === "wide" && (
        <>
          <Panel header="Settings" className="flex-none w-[240px]">
            {optionsPanel}
          </Panel>
          <Panel header="Source" className="flex-1">
            {sourcePanel}
          </Panel>
          <Tabs defaultActiveTab="formatted" className="flex-1">
            <Tab id="formatted" label="Formatted">
              {formattedPanel}
            </Tab>
            <Tab id="ast" label="AST">
              {astPanel}
            </Tab>
            <Tab id="ir" label="Pretty IR">
              {irPanel}
            </Tab>
          </Tabs>
        </>
      )}

      {/* Thin Layout: 1 Column (Full Width) */}
      {screenSize === "thin" && (
        <Tabs defaultActiveTab="source" className="flex-1">
          <Tab id="options" label="Settings">
            {optionsPanel}
          </Tab>
          <Tab id="source" label="Source">
            {sourcePanel}
          </Tab>
          <Tab id="formatted" label="Formatted">
            {formattedPanel}
          </Tab>
          <Tab id="ast" label="AST">
            {astPanel}
          </Tab>
          <Tab id="ir" label="Pretty IR">
            {irPanel}
          </Tab>
        </Tabs>
      )}
    </div>
  );
}
