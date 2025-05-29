import type React from "react";
import type { ScreenSizeType } from "../types";
import { Panel } from "./Panel";
import { Tab, Tabs } from "./Tabs";

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
      {/* Wide Layout: 3 Columns */}
      {screenSize === "wide" && (
        <>
          <Panel header="Format Options" className="w-[240px] flex-none">
            {optionsPanel}
          </Panel>
          <Panel header="Source Code" className="flex-1">
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

      {/* Medium Layout: 2 Columns (Equal 1:1) */}
      {screenSize === "medium" && (
        <>
          <Panel className="flex-1">
            <Tabs defaultActiveTab="source">
              <Tab id="options" label="Options">
                {optionsPanel}
              </Tab>
              <Tab id="source" label="Source">
                {sourcePanel}
              </Tab>
            </Tabs>
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
          <Tab id="options" label="Options">
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
