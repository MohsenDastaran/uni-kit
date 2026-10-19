//! `time-field` — inline time columns and an outline trigger with a popover
//! picker, including seconds, 12-hour clocks and a calendar schedule.
import { signal, Text, View } from "nsh";
import { Button } from "../components/button";
import { Calendar } from "../components/calendar";
import { TimeField } from "../components/time_field";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const time = signal("09:30");
  const seconds = signal("09:30:15");
  const twelve = signal("14:30");
  const changes = signal("09:30");
  const standup = signal("09:45");
  const standupOpen = signal(true);
  const popoverSeconds = signal("14:45:30");
  const meeting = signal("14:30");
  const scheduleTime = signal("09:00");
  const date = signal("2026-10-14");
  const stage = (children: any, width = 340) => <View style={{ width, maxWidth: "100%" }}>{children}</View>;

  // section: Usage
  const usage = stage(
    <View style={{ gap: 8 }}>
      <TimeField value={time()} onChange={(next) => time.set(next)} label="Meeting time" description="Choose the hour and minute." />
      <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Meeting at {time()}</Text>
    </View>,
    320,
  );
  // section: Basic Time Field
  const basic = stage(
    <View style={{ padding: 10, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover }}>
      <TimeField value={time()} onChange={(next) => time.set(next)} showActions />
    </View>,
    320,
  );
  // section: With Seconds
  const withSeconds = stage(
    <View style={{ padding: 10, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover }}>
      <TimeField value={seconds()} onChange={(next) => seconds.set(next)} seconds showActions />
    </View>,
    360,
  );
  // section: 12-Hour Clock
  const twelveHour = stage(
    <View style={{ padding: 10, borderWidth: 1, borderColor: tokens.border, borderRadius: 10, backgroundColor: tokens.popover }}>
      <TimeField value={twelve()} onChange={(next) => twelve.set(next)} hour12 showActions />
    </View>,
    340,
  );
  // section: Sizes, Disabled and Invalid
  const states = (
    <View style={{ width: 360, maxWidth: "100%", gap: 10 }}>
      <TimeField value="08:00" size="small" variant="popover" label="Small" />
      <TimeField value="08:30" size="large" variant="popover" label="Large" />
      <TimeField value="08:45" disabled label="Disabled" description="This field cannot be changed." />
      <TimeField value="23:45" invalid label="Invalid" error="Choose a valid time." />
    </View>
  );
  // section: Handle Changes
  const events = stage(
    <View style={{ gap: 8 }}>
      <TimeField value={changes()} onChange={(next) => changes.set(next)} label="Start time" showActions />
      <Status>Selected: {changes()}</Status>
    </View>,
    320,
  );
  // section: Outline trigger in a label
  const trigger = stage(
    <TimeField
      value={standup()}
      onChange={(next) => standup.set(next)}
      variant="popover"
      open={standupOpen()}
      onOpenChange={(next) => standupOpen.set(next)}
      label="Standup time"
      description="Select a time before the meeting starts."
      placeholder="Select time"
    />,
    340,
  );
  // section: Popover with seconds
  const popoverSecondsView = stage(
    <TimeField
      value={popoverSeconds()}
      onChange={(next) => popoverSeconds.set(next)}
      variant="popover"
      seconds
      hour12
      label="Stream goes live"
      description="The countdown starts at this time."
    />,
    340,
  );
  // section: Meeting time card
  const meetingCard = (
    <View style={{ width: 340, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover, overflow: "hidden" }}>
      <TimeField value={meeting()} onChange={(next) => meeting.set(next)} showActions={false} />
      <View style={{ padding: 12, borderTopWidth: 1, borderTopColor: tokens.border, gap: 8 }}>
        <Text style={{ fontSize: 12, color: tokens.foreground }}>Meeting at {meeting()}</Text>
        <Button label="Save time" size="small" variant="primary" onPress={() => {}} style={{ alignSelf: "flex-start" }} />
      </View>
    </View>
  );
  // section: Schedule a post
  const schedule = (
    <View style={{ width: 650, maxWidth: "100%", borderWidth: 1, borderColor: tokens.border, borderRadius: 12, backgroundColor: tokens.popover, overflow: "hidden" }}>
      <View style={{ padding: 14, gap: 3 }}>
        <Text style={{ fontSize: 15, fontWeight: "600", color: tokens.foreground }}>Schedule a post</Text>
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>Pick the day and the time it goes live.</Text>
      </View>
      <View style={{ flexDirection: "row", gap: 10, paddingHorizontal: 14, alignItems: "flex-start" }}>
        <View style={{ flex: 1 }}><Calendar mode="single" value={date()} onSelect={(next) => date.set(next)} card={false} /></View>
        <TimeField value={scheduleTime()} onChange={(next) => scheduleTime.set(next)} showActions={false} style={{ width: 260 }} />
      </View>
      <View style={{ padding: 12, borderTopWidth: 1, borderTopColor: tokens.border, flexDirection: "row", alignItems: "center", gap: 8 }}>
        <Text style={{ flex: 1, fontSize: 11, fontWeight: "500", color: tokens.foreground }}>{date()} at {scheduleTime()}</Text>
        <Button label="Schedule" size="small" variant="primary" onPress={() => {}} />
      </View>
    </View>
  );
  // section: end
  return (
    <Page>
      <Caption>TimeField uses hour and minute columns; the popover keeps the same picker behind a labelled trigger.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Basic Time Field">{basic}</ExampleSection>
      <ExampleSection index={2} title="With Seconds">{withSeconds}</ExampleSection>
      <ExampleSection index={3} title="12-Hour Clock">{twelveHour}</ExampleSection>
      <ExampleSection index={4} title="Sizes, Disabled and Invalid">{states}</ExampleSection>
      <ExampleSection index={5} title="Handle Changes">{events}</ExampleSection>
      <ExampleSection index={6} title="Outline trigger in a label">{trigger}</ExampleSection>
      <ExampleSection index={7} title="Popover with seconds">{popoverSecondsView}</ExampleSection>
      <ExampleSection index={8} title="Meeting time card">{meetingCard}</ExampleSection>
      <ExampleSection index={9} title="Schedule a post">{schedule}</ExampleSection>
    </Page>
  );
}
