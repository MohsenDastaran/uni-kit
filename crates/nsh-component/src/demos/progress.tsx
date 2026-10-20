//! `progress` — linear and circular completion indicators, including animated
//! and looping progress driven by application state.
import { cleanup, effect, signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { Progress, ProgressCircle } from "../components/progress";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

export function Demo() {
  const value = signal(40);
  const animated = signal(0);
  const looping = signal(0);

  effect(() => {
    const grow = setInterval(() => animated.update((current) => (current >= 72 ? 72 : current + 1)), 60);
    const loop = setInterval(() => looping.update((current) => (current >= 100 ? 0 : current + 2)), 70);
    cleanup(() => {
      clearInterval(grow);
      clearInterval(loop);
    });
  });

  // section: Usage
  const usage = (
    <View style={{ width: 420, maxWidth: "100%" }}>
      <Progress value={value()} label="Workspace setup" description="Downloading assets…" />
    </View>
  );

  // section: Progress basics
  const basic = <Progress value={70} label="Large progress" style={{ width: 420, maxWidth: "100%" }} />;

  // section: Values
  const values = (
    <View style={{ width: 420, maxWidth: "100%", gap: 12 }}>
      <Progress value={0} label="Not started" />
      <Progress value={25} label="Quarter" />
      <Progress value={50} label="Halfway" />
      <Progress value={75} label="Almost there" />
      <Progress value={100} label="Complete" status="success" />
    </View>
  );

  // section: Sizes and status colors
  const sizes = (
    <View style={{ width: 420, maxWidth: "100%", gap: 12 }}>
      <Progress value={60} size="small" label="Small" />
      <Progress value={60} size="large" label="Large" />
      <Progress value={60} status="success" label="Success" />
      <Progress value={60} status="warning" label="Warning" />
      <Progress value={60} status="danger" label="Danger" />
    </View>
  );

  // section: Progress circle
  const circle = (
    <Row gap={18}>
      <ProgressCircle value={value()} />
      <ProgressCircle value={75} status="success" />
      <ProgressCircle value={40} status="danger" />
      <Progress circle value={64} innerContent={<View style={{ alignItems: "center", gap: 1 }}><Text style={{ fontSize: 14, fontWeight: "600", color: tokens.foreground }}>64%</Text><Text style={{ fontSize: 8, color: tokens.mutedForeground }}>Synced</Text></View>} />
    </Row>
  );

  // section: Large progress
  const large = (
    <Progress value={70} label="Large progress" size="large" style={{ width: 420 }} />
  );

  // section: Progress bar with status messages
  const status = (
    <View style={{ width: 420, maxWidth: "100%", gap: 10 }}>
      <Progress value={60} label="Workspace Setup" description="Downloading assets…" />
      <Progress value={84} label="Restoring workspace" description="Bringing back open tabs and extensions." status="info" />
    </View>
  );

  // section: File upload list
  const files = (
    <View style={{ width: 460, maxWidth: "100%", gap: 12 }}>
      {[
        { name: "document.pdf", value: 24, time: "2m 30s" },
        { name: "presentation.pptx", value: 48, time: "45s" },
        { name: "spreadsheet.xlsx", value: 12, time: "5m 12s" },
        { name: "image.jpg", value: 100, time: "Complete" },
      ].map((file) => (
        <Row gap={9} style={{ alignItems: "center" }}>
          <Icon name="file-text" size={13} color={tokens.mutedForeground} />
          <Text style={{ width: 130, fontSize: 11, color: tokens.foreground }}>{file.name}</Text>
          <Progress value={file.value} showValue={false} size="small" style={{ flex: 1 }} />
          <Text style={{ width: 64, fontSize: 10, color: tokens.mutedForeground, textAlign: "right" }}>{file.time}</Text>
        </Row>
      ))}
    </View>
  );

  // section: Progress bar with slider
  const slider = (
    <View style={{ width: 420, maxWidth: "100%", gap: 12 }}>
      <Progress value={42} showValue={false} thumb />
      <Progress value={78} showValue={false} thumb status="info" />
    </View>
  );

  // section: Setup progress
  const steps = (
    <View style={{ width: 420, maxWidth: "100%", gap: 10 }}>
      <View style={{ flexDirection: "row", alignItems: "center" }}>
        <Text style={{ flex: 1, fontSize: 12, fontWeight: "600", color: tokens.foreground }}>Setup Progress</Text>
        <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>2 of 4 steps</Text>
      </View>
      <Progress value={50} showValue={false} />
      {[
        { label: "Account", done: true },
        { label: "Profile", done: true },
        { label: "Preferences", done: false },
        { label: "Review", done: false },
      ].map((step) => (
        <Row gap={7} style={{ alignItems: "center" }}>
          <View style={{ width: 14, height: 14, borderRadius: 7, borderWidth: 1, borderColor: step.done ? tokens.success : tokens.border, alignItems: "center", justifyContent: "center" }}>
            {step.done ? <Icon name="check" size={9} color={tokens.success} /> : null}
          </View>
          <Text style={{ fontSize: 11, color: step.done ? tokens.foreground : tokens.mutedForeground }}>{step.label}</Text>
        </Row>
      ))}
    </View>
  );

  // section: Colored progress bars
  const colored = (
    <View style={{ width: 460, maxWidth: "100%", gap: 10 }}>
      <Progress value={82} showValue={false} status="success" size="small" />
      <Progress value={74} showValue={false} status="warning" size="small" />
      <Progress value={66} showValue={false} status="info" size="small" />
      <Progress value={58} showValue={false} status="danger" size="small" />
      <Progress value={44} showValue={false} status="default" size="small" />
    </View>
  );

  // section: Animated progress
  const animatedProgress = (
    <View style={{ width: 420, maxWidth: "100%" }}>
      <Progress value={animated()} label="Animated progress" description="Animates to 72% when it appears." status="info" />
    </View>
  );

  // section: Looping progress
  const loopingProgress = (
    <View style={{ width: 420, maxWidth: "100%", gap: 6 }}>
      <Progress value={looping()} label="Looping progress" showValue={false} status="success" description="Cycles continuously while the task is active." />
      <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{looping()}% complete</Text>
    </View>
  );

  // section: Progress API
  const api = <Progress value={50} label="API progress" style={{ width: 420 }} />;

  // section: Custom Style
  const custom = (
    <View style={{ width: 420, maxWidth: "100%", gap: 10 }}>
      <Progress value={value()} label="Uploading" status="info" />
      <Progress value={value()} label="Custom fill" size="small" showValue={false} style={{ padding: 6, borderWidth: 1, borderColor: tokens.border, borderRadius: 8, backgroundColor: tokens.muted }} />
      <Row gap={8}>
        <Button label="Less" size="small" onPress={() => value.update((n) => Math.max(0, n - 10))} />
        <Button label="More" size="small" variant="primary" onPress={() => value.update((n) => Math.min(100, n + 10))} />
      </Row>
    </View>
  );
  // section: end

  return (
    <Page>
      <Caption>
        Progress shows completion; a spinner is for work with no measurable end.
        The last two examples animate from live application state.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Progress basics">{basic}</ExampleSection>
      <ExampleSection index={2} title="Values">{values}</ExampleSection>
      <ExampleSection index={3} title="Sizes and status colors">{sizes}</ExampleSection>
      <ExampleSection index={4} title="Progress circle">{circle}</ExampleSection>
      <ExampleSection index={5} title="Large progress">{large}</ExampleSection>
      <ExampleSection index={6} title="Progress bar with status messages">{status}</ExampleSection>
      <ExampleSection index={7} title="File upload list">{files}</ExampleSection>
      <ExampleSection index={8} title="Progress bar with slider">{slider}</ExampleSection>
      <ExampleSection index={9} title="Setup progress">{steps}</ExampleSection>
      <ExampleSection index={10} title="Colored progress bars">{colored}</ExampleSection>
      <ExampleSection index={11} title="Animated progress">{animatedProgress}</ExampleSection>
      <ExampleSection index={12} title="Looping progress">{loopingProgress}</ExampleSection>
      <ExampleSection index={13} title="Custom Style">{custom}</ExampleSection>
      <ExampleSection index={14} title="Progress API">{api}</ExampleSection>
    </Page>
  );
}
