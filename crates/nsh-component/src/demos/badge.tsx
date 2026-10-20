//! `badge` — ReUI-style pills and the anchored count/dot/icon markers.
//!
//! The first sections are standalone badges: tinted and solid appearances,
//! sizes, full-radius counts, a leading icon, a dismiss button, a status dot, a
//! link, an avatar and an image/flag slot. The last two keep the original
//! overlay behaviour: a count, dot or glyph hanging off an avatar or an icon.
import { signal, View } from "nsh";
import { Image } from "nsh/image";
import { LayoutAnimation } from "nsh/animated";
import { reduceMotion } from "nsh/accessibility";
import cover from "../../examples/assets/cover.png";
import { Avatar } from "../components/avatar";
import { Badge } from "../components/badge";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row, Status } from "./ui";

export function Demo() {
  const opened = signal("No link opened yet");
  const tags = signal(["Design", "Engineering", "Marketing"]);
  const person = signal(true);
  const removeTag = (tag: string) => {
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    tags.set(tags().filter((item) => item !== tag));
  };
  const restoreTags = () => {
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    tags.set(["Design", "Engineering", "Marketing"]);
  };
  const dismissPerson = () => {
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    person.set(false);
  };
  // section: Usage
  const usage = (
    <Row gap={8}>
      <Badge label="New" variant="success" />
      <Badge count={3} variant="info" />
      <Badge label="Draft" variant="secondary" />
      <Badge dot label="Online" variant="success" />
    </Row>
  );

  // section: Warning light badge
  const warning = (
    <Row gap={8}>
      <Badge label="Needs review" variant="warning" />
      <Badge label="Expiring soon" icon="clock" variant="warning" />
      <Badge label="3 warnings" variant="warning" appearance="solid" />
    </Row>
  );

  // section: Badge size variations
  const sizes = (
    <Row gap={8} style={{ alignItems: "center" }}>
      <Badge label="Small" size="small" variant="info" />
      <Badge label="Default" size="medium" variant="info" />
      <Badge label="Large" size="large" variant="info" />
      <Badge label="Small solid" size="small" variant="success" appearance="solid" />
      <Badge label="Large solid" size="large" variant="success" appearance="solid" />
    </Row>
  );

  // section: Full-radius badges
  const fullRadius = (
    <Row gap={8}>
      <Badge count={8} variant="info" radius="full" />
      <Badge count={128} max={99} variant="success" radius="full" appearance="solid" />
      <Badge label="Inbox" variant="secondary" radius="full" />
      <Badge label="Open" variant="default" radius="full" />
      <Badge label="Outline" variant="outline" radius="full" />
    </Row>
  );

  // section: Badge with icon
  const badgeIcon = (
    <Row gap={8}>
      <Badge icon="check" label="Verified" variant="success" />
      <Badge icon="sparkles" label="Pro" variant="info" appearance="solid" />
      <Badge icon="star" label="Featured" variant="warning" />
      <Badge icon="circle-check" label="Complete" variant="outline" />
    </Row>
  );

  // section: Badge with dismiss
  const dismiss = (
    <Row gap={8}>
      {tags().map((tag) => (
        <Badge
          label={tag}
          variant="secondary"
          closable
          onRemove={() => removeTag(tag)}
        />
      ))}
      {tags().length === 0 ? (
        <Button label="Restore badges" size="small" onPress={restoreTags} />
      ) : null}
    </Row>
  );

  // section: Badge with status dot
  const status = (
    <Row gap={8}>
      <Badge dot label="Online" variant="success" />
      <Badge dot label="Away" variant="warning" />
      <Badge dot label="Busy" variant="danger" />
      <Badge dot label="Offline" variant="secondary" />
      <Badge dot label="Guest" variant="info" />
    </Row>
  );

  // section: Badge rendered as a link
  const link = (
    <Row gap={8}>
      <Badge
        label="View pull request"
        icon="external-link"
        variant="outline"
        onPress={() => opened.set("Opened pull request #42")}
        accessibilityLabel="Open pull request 42"
      />
      <Badge
        label="Read the docs"
        variant="info"
        onPress={() => opened.set("Opened the documentation")}
      />
      <Badge
        label="Repository"
        icon="github"
        variant="secondary"
        onPress={() => opened.set("Opened the repository")}
        accessibilityLabel="Open the repository"
      />
      <Status>{opened()}</Status>
    </Row>
  );

  // section: Badge with avatar
  const people = (
    <Row gap={8}>
      {person() ? (
        <Badge
          leading={<Avatar name="Ada Lovelace" size="xs" />}
          label="Ada Lovelace"
          variant="secondary"
          closable
          onRemove={dismissPerson}
        />
      ) : (
        <Badge label="Ada Lovelace dismissed" variant="outline" />
      )}
      <Badge
        leading={<Avatar name="Grace Hopper" size="xs" />}
        label="Grace Hopper"
        variant="secondary"
      />
      <Badge
        leading={<Avatar name="Alan Turing" size="xs" />}
        label="Alan Turing"
        variant="outline"
      />
    </Row>
  );

  // section: Badge with flag/image
  const image = (
    <Row gap={8}>
      <Badge
        leading={
          <Image
            src={cover}
            resizeMode="cover"
            style={{ width: 16, height: 16, borderRadius: 3 }}
          />
        }
        label="Mountain"
        variant="secondary"
      />
      <Badge
        leading={
          <View
            style={{
              flexDirection: "row",
              width: 18,
              height: 12,
              borderWidth: 1,
              borderColor: tokens.border,
              borderRadius: 2,
              overflow: "hidden",
            }}
          >
            <View style={{ flex: 1, backgroundColor: tokens.info }} />
            <View style={{ flex: 1, backgroundColor: tokens.background }} />
            <View style={{ flex: 1, backgroundColor: tokens.danger }} />
          </View>
        }
        label="Flag"
        variant="outline"
      />
      <Badge
        leading={<Icon name="globe" size={14} color={tokens.mutedForeground} />}
        label="Image slot"
        variant="secondary"
      />
    </Row>
  );

  // section: Notification Indicators
  const notifications = (
    <Row gap={24}>
      <Badge count={3}><Icon name="bell" size={28} color={tokens.foreground} /></Badge>
      <Badge count={8}><Icon name="message-circle" size={28} color={tokens.foreground} /></Badge>
      <Badge icon="info" color={tokens.info}><Icon name="inbox" size={28} color={tokens.foreground} /></Badge>
      <Badge dot color={tokens.success}>
        <Avatar name="Ada Lovelace" size="lg" />
      </Badge>
    </Row>
  );

  // section: Count Formatting
  const formatting = (
    <Row gap={20}>
      <Badge count={0}>
        <Avatar name="Ada Lovelace" size="lg" />
      </Badge>
      <Badge count={9} max={9}>
        <Avatar name="Ada Lovelace" size="lg" />
      </Badge>
      <Badge count={99} max={99}>
        <Avatar name="Ada Lovelace" size="lg" />
      </Badge>
      <Badge count={1000} max={99}>
        <Avatar name="Ada Lovelace" size="lg" />
      </Badge>
    </Row>
  );
  // section: end

  return (
    <Page>
      <Caption>
        A badge is a compact pill for status and counts, or an overlay hung past the
        top-right corner of the avatar or icon it marks. The pill keeps one leading
        slot, a quiet tint and a 150 ms hover/press transition; the overlay leaves the
        child's layout alone.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Warning light badge">{warning}</ExampleSection>
      <ExampleSection index={2} title="Badge size variations">{sizes}</ExampleSection>
      <ExampleSection index={3} title="Full-radius badges">{fullRadius}</ExampleSection>
      <ExampleSection index={4} title="Badge with icon">{badgeIcon}</ExampleSection>
      <ExampleSection index={5} title="Badge with dismiss">{dismiss}</ExampleSection>
      <ExampleSection index={6} title="Badge with status dot">{status}</ExampleSection>
      <ExampleSection index={7} title="Badge rendered as a link">{link}</ExampleSection>
      <ExampleSection index={8} title="Badge with avatar">{people}</ExampleSection>
      <ExampleSection index={9} title="Badge with flag/image">{image}</ExampleSection>
      <ExampleSection index={10} title="Notification Indicators">{notifications}</ExampleSection>
      <ExampleSection index={11} title="Count Formatting">{formatting}</ExampleSection>
    </Page>
  );
}
