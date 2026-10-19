//! `marker` — separators, border markers, tool-call logs and agent-run phases.
import { Text, View } from "nsh";
import { Marker, MarkerContent, MarkerGroup } from "../components/marker";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

export function Demo() {
  // section: Usage
  const usage = (
    <Marker separator label="Today" style={{ width: 420, maxWidth: "100%" }} />
  );

  // section: Plain
  const plain = (
    <Marker icon="info" label="Conversation started" description="Just now" style={{ width: 420 }} />
  );

  // section: Separator
  const separator = (
    <View style={{ width: 420, maxWidth: "100%", gap: 8 }}>
      <Text style={{ fontSize: 11, color: tokens.foreground }}>Read 14 files under src/billing</Text>
      <Marker separator label="Planning" />
      <Text style={{ fontSize: 11, color: tokens.foreground }}>Wrote 3 files, 128 lines changed</Text>
    </View>
  );

  // section: Border
  const border = (
    <Marker border icon="terminal" label="Read 14 files under src/billing" description="2 seconds ago" style={{ width: 420 }} />
  );

  // section: Alignment
  const alignment = (
    <View style={{ width: 420, maxWidth: "100%", gap: 6 }}>
      <Marker align="start" label="Start marker" />
      <Marker align="center" label="Centered marker" />
      <Marker align="end" label="End marker" />
    </View>
  );

  // section: Loading styles
  const loading = (
    <View style={{ width: 420, maxWidth: "100%", gap: 6 }}>
      <Marker loading label="Loading messages…" />
      <Marker status="pending" label="Re-running the test suite" />
      <Marker status="success" label="Checked out fix/rate-limit-window" />
    </View>
  );

  // section: Icons, content, and interactive children
  const icons = (
    <Marker icon="message-circle" style={{ width: 440 }}>
      <MarkerContent>
        <Text style={{ fontSize: 11, color: tokens.foreground }}>A system note with an icon</Text>
        <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>Use children for rich content.</Text>
      </MarkerContent>
    </Marker>
  );

  // section: Tool call log
  const toolLog = (
    <MarkerGroup title="Agent run" description="fix/rate-limit-window · 1m 48s" style={{ width: 460 }}>
      <Marker icon="search" label="Searched 214 files for rateLimit" />
      <Marker icon="file-text" label="Read src/lib/rate-limit.ts" />
      <Marker icon="pencil" label="Edited src/lib/rate-limit.ts, 2 hunks" />
      <Marker icon="terminal" label="Ran pnpm test rate-limit" />
      <Marker icon="file-text" label="Read the Upstash quota docs" />
    </MarkerGroup>
  );

  // section: Agent run with live status
  const liveRun = (
    <MarkerGroup title="Agent run" description="fix/rate-limit-window" style={{ width: 420 }}>
      <Marker status="success" label="Read src/lib/rate-limit.ts" />
      <Marker status="success" label="Reproduced the 429 on staging" />
      <Marker status="success" label="Widened the window to 60s" />
      <Marker status="danger" label="Type check failed in src/db/schema.ts" badge="failed" badgeTone="danger" />
      <Marker status="pending" label="Re-running the test suite" />
    </MarkerGroup>
  );

  // section: Agent run grouped by phase
  const phases = (
    <MarkerGroup title="Agent run" description="auto-rate-limit, 1m 48s" style={{ width: 460 }}>
      <Marker separator label="Plan" />
      <Marker icon="search" label="Searched 214 files for rateLimit(" />
      <Marker icon="file-text" label="Read src/lib/rate-limit.ts" />
      <Marker icon="file-text" label="Drafted a 3 step plan" />
      <Marker separator label="Edit" />
      <Marker icon="pencil" label="Widened the window to 60s" />
      <Marker icon="code" label="Wired the limiter into the send route" />
      <Marker icon="code" label="Committed to fix/rate-limit-window" />
      <Marker separator label="Verify" />
      <Marker icon="terminal" label="Ran pnpm test rate-limit" />
      <Marker status="info" label="Confirmed the 429 no longer fires" />
    </MarkerGroup>
  );

  // section: File change markers
  const diff = (
    <View style={{ width: 460, maxWidth: "100%", gap: 4 }}>
      {[
        { name: "src/lib/rate-limit.ts", change: "+64", badgeTone: "success" as const, icon: "plus" as const },
        { name: "src/app/api/send/route.ts", change: "+18 -4", badgeTone: "info" as const, icon: "pencil" as const },
        { name: "src/db/schema.ts", change: "+6 -1", badgeTone: "info" as const, icon: "pencil" as const },
        { name: "src/lib/legacy-throttle.ts", change: "-91", badgeTone: "danger" as const, icon: "x" as const },
        { name: "README.md", change: "+3", badgeTone: "info" as const, icon: "file-text" as const },
      ].map((file) => (
        <Marker icon={file.icon} label={file.name} badge={file.change} badgeTone={file.badgeTone} />
      ))}
    </View>
  );

  // section: Review actions
  const actions = (
    <View style={{ width: 420, maxWidth: "100%", gap: 4 }}>
      <Marker icon="search" label="Review pull request #412" />
      <Marker icon="file-text" label="Open the diff for rate-limit.ts" />
      <Marker icon="refresh-cw" label="Revert this change" />
    </View>
  );

  // section: Custom styling
  const custom = (
    <View style={{ width: 420, maxWidth: "100%", gap: 8 }}>
      <Marker border icon="circle-alert" iconTone="warning" tone="warning" label="Heads up" description="One warning found." />
      <Marker separator label="Build output" style={{ width: 420 }} />
      <Marker border icon="check" iconTone="success" tone="success" label="Build passed" description="All checks completed." />
    </View>
  );
  // section: Variants
  const variants = (
    <View style={{ width: 440, maxWidth: "100%", gap: 8 }}>
      <Marker icon="info" label="Plain marker" />
      <Marker separator label="Separator" />
      <Marker border icon="terminal" label="Bordered marker" description="A note with its own surface." />
    </View>
  );

  // section: end

  return (
    <Page>
      <Caption>
        Markers sit between messages or list sections, or form the rows of an
        agent run. Use a separator for phases; use a border for a note.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Plain">{plain}</ExampleSection>
      <ExampleSection index={2} title="Separator">{separator}</ExampleSection>
      <ExampleSection index={3} title="Border">{border}</ExampleSection>
      <ExampleSection index={4} title="Alignment">{alignment}</ExampleSection>
      <ExampleSection index={5} title="Loading styles">{loading}</ExampleSection>
      <ExampleSection index={6} title="Icons, content, and interactive children">{icons}</ExampleSection>
      <ExampleSection index={7} title="Tool call log">{toolLog}</ExampleSection>
      <ExampleSection index={8} title="Agent run with live status">{liveRun}</ExampleSection>
      <ExampleSection index={9} title="Agent run grouped by phase">{phases}</ExampleSection>
      <ExampleSection index={10} title="File change markers">{diff}</ExampleSection>
      <ExampleSection index={11} title="Review actions">{actions}</ExampleSection>
      <ExampleSection index={12} title="Custom styling">{custom}</ExampleSection>
      <ExampleSection index={13} title="Variants">{variants}</ExampleSection>
    </Page>
  );
}
