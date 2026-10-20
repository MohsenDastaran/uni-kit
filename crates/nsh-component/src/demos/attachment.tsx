//! `attachment` — a file/media card with lifecycle, preview and actions.
import { Scroll, signal, Text, View } from "nsh";
import { LayoutAnimation } from "nsh/animated";
import { reduceMotion } from "nsh/accessibility";
import { Attachment, AttachmentGroup, type AttachmentStatus } from "../components/attachment";
import { Button } from "../components/button";
import { Icon } from "../components/icon";
import { type IconName } from "../components/icons";
import { Input } from "../components/input";
import { alpha, tokens } from "../components/theme";
import cover from "../../examples/assets/cover.png";
import { Caption, Column, ExampleSection, Page, Row, Status } from "./ui";

type QueueEntry = {
  id: string;
  name: string;
  meta: string;
  status: AttachmentStatus;
  progress?: number;
};

const initialQueue = (): QueueEntry[] => [
  { id: "q1", name: "brand-guide.pdf", meta: "PDF · 2.6 MB", status: "complete", progress: 100 },
  { id: "q2", name: "hero-shot.png", meta: "PNG · 4.1 MB", status: "uploading", progress: 46 },
  { id: "q3", name: "contract-final.pdf", meta: "PDF · 880 KB", status: "scanning", progress: 72 },
  { id: "q4", name: "datasets.zip", meta: "ZIP · 18 MB", status: "error" },
  { id: "q5", name: "meeting-notes.txt", meta: "TXT · 12 KB", status: "idle" },
];

type TrayEntry = { id: string; name: string; icon: IconName };

const initialTray = (): TrayEntry[] => [
  { id: "t1", name: "brief.pdf", icon: "file-text" },
  { id: "t2", name: "cover.png", icon: "image" },
  { id: "t3", name: "notes.txt", icon: "file-text" },
];

const pickerPool: TrayEntry[] = [
  { id: "p1", name: "research-notes.md", icon: "file-text" },
  { id: "p2", name: "screenshot.png", icon: "image" },
  { id: "p3", name: "handoff.zip", icon: "file" },
  { id: "p4", name: "budget.csv", icon: "table" },
];

export function Demo() {
  // section: Usage
  const usage = (
    <Attachment
      name="quarterly-report.pdf"
      meta="PDF · 1.9 MB"
      icon="file-text"
      status="complete"
      onDownload={() => {}}
      onRemove={() => {}}
      style={{ maxWidth: 360 }}
    />
  );

  // section: Anatomy and basic usage
  const anatomy = (
    <Column gap={8}>
      <Attachment
        name="design-spec.pdf"
        meta="PDF · 2.4 MB"
        icon="file-text"
        onDownload={() => {}}
      />
      <Attachment
        name="notes.txt"
        meta="TXT · 12 KB"
        icon="file-text"
        status="complete"
        onRemove={() => {}}
        style={{ maxWidth: 360 }}
      />
    </Column>
  );

  // section: Lifecycle states
  const states = (
    <Column gap={8}>
      <Attachment name="ready-to-send.pdf" meta="PDF · 820 KB" icon="file-text" onRemove={() => {}} />
      <Attachment
        name="uploading.pdf"
        meta="PDF · 3.2 MB"
        status="uploading"
        progress={62}
        onRemove={() => {}}
      />
      <Attachment
        name="scanning.pdf"
        meta="PDF · 1.4 MB"
        status="scanning"
        progress={38}
        onRemove={() => {}}
      />
      <Attachment
        name="done.pdf"
        meta="PDF · 1.2 MB"
        status="complete"
        onDownload={() => {}}
      />
      <Attachment
        name="failed.pdf"
        meta="PDF · 4.6 MB"
        status="error"
        onRetry={() => {}}
        onRemove={() => {}}
      />
    </Column>
  );

  // section: Upload queue across all states
  const queue = signal<QueueEntry[]>(initialQueue());
  // Retry restarts an upload, then scans it, then completes it.
  let queueTimer: number | undefined;
  const stepQueue = (id: string) => {
    let scanned = false;
    queue.update((entries) =>
      entries.map((entry) => {
        if (entry.id !== id || entry.status !== "uploading") return entry;
        const next = Math.min(100, (entry.progress ?? 0) + 9);
        if (next >= 100) {
          scanned = true;
          return { ...entry, status: "scanning", progress: 62 };
        }
        return { ...entry, progress: next };
      }),
    );
    if (scanned) {
      queueTimer = setTimeout(() => {
        queue.update((entries) =>
          entries.map((entry) =>
            entry.id === id ? { ...entry, status: "complete", progress: 100 } : entry,
          ),
        );
      }, 900);
    } else {
      queueTimer = setTimeout(() => stepQueue(id), 220);
    }
  };
  const removeQueue = (id: string) => {
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    queue.update((entries) => entries.filter((entry) => entry.id !== id));
  };
  const retryQueue = (id: string) => {
    queue.update((entries) =>
      entries.map((entry) =>
        entry.id === id ? { ...entry, status: "uploading", progress: 8 } : entry,
      ),
    );
    if (queueTimer !== undefined) clearTimeout(queueTimer);
    queueTimer = setTimeout(() => stepQueue(id), 240);
  };
  const resetQueue = () => {
    if (queueTimer !== undefined) clearTimeout(queueTimer);
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    queue.set(initialQueue());
  };
  const uploadQueue = (
    <Column gap={8}>
      {queue().map((entry) => (
        <Attachment
          name={entry.name}
          meta={entry.meta}
          status={entry.status}
          progress={entry.progress}
          onRetry={entry.status === "error" ? () => retryQueue(entry.id) : undefined}
          onRemove={() => removeQueue(entry.id)}
          onDownload={entry.status === "complete" ? () => {} : undefined}
        />
      ))}
      <Row>
        <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
          {queue().length} files in the queue
        </Text>
        <Button label="Reset queue" size="small" variant="ghost" onPress={resetQueue} />
      </Row>
    </Column>
  );

  // section: Media and image previews
  const media = (
    <Column gap={10}>
      <Attachment
        layout="stacked"
        name="studio-cover.png"
        meta="PNG · 1920 × 1080"
        image={cover}
        status="complete"
        onDownload={() => {}}
        style={{ width: 300 }}
      />
      <Scroll
        horizontal
        snapToInterval={158}
        showsIndicator={false}
        style={{ height: 180, borderRadius: 10 }}
      >
        <View style={{ flexDirection: "row", gap: 8, paddingRight: 8 }}>
          {["hero.png", "detail.png", "texture.png", "mockup.png"].map((label, index) => (
            <Attachment
              layout="stacked"
              name={label}
              meta={`PNG · ${1.2 + index * 0.4} MB`}
              image={cover}
              status="complete"
              onRemove={() => {}}
              style={{ width: 150 }}
            />
          ))}
        </View>
      </Scroll>
    </Column>
  );

  // section: Sizes and axes
  const sizes = (
    <Column gap={10}>
      <Attachment
        size="large"
        name="funding-proposal.pdf"
        meta="PDF · 3.8 MB · 24 pages"
        icon="file-text"
        status="complete"
        onDownload={() => {}}
        onRemove={() => {}}
        style={{ maxWidth: 420 }}
      />
      <Attachment
        size="medium"
        name="design-assets.zip"
        meta="ZIP · 48 MB"
        icon="file"
        status="uploading"
        progress={34}
        onRemove={() => {}}
        style={{ maxWidth: 360 }}
      />
      <Attachment
        size="small"
        name="agenda.docx"
        meta="DOCX · 64 KB"
        icon="file-text"
        onDownload={() => {}}
        style={{ maxWidth: 320 }}
      />
      <Row>
        <Attachment variant="chip" name="auth.ts · lines 42–58" icon="code-xml" />
        <Attachment variant="chip" name="invoice.pdf" icon="file-text" onRemove={() => {}} />
      </Row>
    </Column>
  );

  // section: Composer tray
  const tray = signal<TrayEntry[]>(initialTray());
  let traySeq = 0;
  const addToTray = () => {
    const next = pickerPool[tray().length % pickerPool.length];
    if (tray().length >= 6) return;
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    tray.update((entries) => [...entries, { ...next, id: `${next.id}-${traySeq++}` }]);
  };
  const removeFromTray = (id: string) => {
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    tray.update((entries) => entries.filter((entry) => entry.id !== id));
  };
  const composer = (
    <View
      style={{
        borderWidth: 1,
        borderColor: tokens.border,
        borderRadius: 10,
        backgroundColor: tokens.background,
        padding: 10,
        gap: 8,
        maxWidth: 560,
      }}
    >
      <View style={{ flexDirection: "row", flexWrap: "wrap", gap: 6 }}>
        {tray().length === 0 ? (
          <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
            No attachments yet.
          </Text>
        ) : null}
        {tray().map((entry) => (
          <Attachment
            variant="chip"
            name={entry.name}
            icon={entry.icon}
            onRemove={() => removeFromTray(entry.id)}
          />
        ))}
      </View>
      <View style={{ flexDirection: "row", alignItems: "center", gap: 8 }}>
        <Icon name="paperclip" size={16} color={tokens.mutedForeground} />
        <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
          {tray().length} of 6 attached
        </Text>
        <View style={{ flex: 1, minWidth: 120 }}>
          <Input size="small" placeholder="Write a message…" />
        </View>
        <Button
          icon="plus"
          size="small"
          variant="ghost"
          accessibilityLabel="Attach another file"
          onPress={addToTray}
        />
        <Button
          icon="send"
          size="small"
          variant="primary"
          accessibilityLabel="Send message"
          onPress={() => {}}
        />
      </View>
    </View>
  );

  // section: File picker
  const picked = signal<TrayEntry[]>([]);
  let pickSeq = 0;
  const pickFile = () => {
    const next = pickerPool[picked().length % pickerPool.length];
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    picked.update((entries) => [...entries, { ...next, id: `${next.id}-${pickSeq++}` }]);
  };
  const clearPicked = () => {
    if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
    picked.set([]);
  };
  const picker = (
    <Column gap={8} style={{ maxWidth: 560 }}>
      {picked().length === 0 ? (
        <View
          style={{
            borderWidth: 1,
            borderStyle: "dashed",
            borderColor: tokens.border,
            borderRadius: 10,
            paddingVertical: 24,
            paddingHorizontal: 16,
            alignItems: "center",
            gap: 6,
            backgroundColor: alpha(tokens.muted, 0.4),
          }}
        >
          <View
            style={{
              width: 36,
              height: 36,
              borderRadius: 18,
              backgroundColor: tokens.muted,
              alignItems: "center",
              justifyContent: "center",
            }}
          >
            <Icon name="upload" size={18} color={tokens.mutedForeground} />
          </View>
          <Text style={{ fontSize: 13, fontWeight: "500", color: tokens.foreground }}>
            Choose files
          </Text>
          <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>
            PDF, PNG, ZIP or CSV up to 20 MB
          </Text>
          <Button label="Choose files" icon="upload" size="small" onPress={pickFile} style={{ alignSelf: "center" }} />
        </View>
      ) : (
        <View
          style={{
            borderWidth: 1,
            borderColor: tokens.border,
            borderRadius: 10,
            padding: 10,
            gap: 8,
          }}
        >
          <Row>
            <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
              {picked().length} selected
            </Text>
            <Button label="Add another" size="small" variant="ghost" onPress={pickFile} />
            <Button label="Clear" size="small" variant="ghost" onPress={clearPicked} />
          </Row>
          <View style={{ flexDirection: "row", flexWrap: "wrap", gap: 6 }}>
            {picked().map((entry) => (
              <Attachment
                variant="chip"
                name={entry.name}
                icon={entry.icon}
                status="complete"
                onRemove={() => {
                  if (!reduceMotion()) LayoutAnimation.easeInEaseOut();
                  picked.update((entries) => entries.filter((item) => item.id !== entry.id));
                }}
              />
            ))}
          </View>
        </View>
      )}
    </Column>
  );

  // section: Groups
  const groups = (
    <Column gap={10}>
      <AttachmentGroup>
        <Attachment name="chapter-01.pdf" meta="PDF · 640 KB" icon="file-text" />
        <Attachment name="chapter-02.pdf" meta="PDF · 710 KB" icon="file-text" />
        <Attachment name="chapter-03.pdf" meta="PDF · 690 KB" icon="file-text" status="uploading" progress={81} />
      </AttachmentGroup>
      <AttachmentGroup direction="row" wrap>
        <Attachment variant="chip" name="spec.pdf" icon="file-text" />
        <Attachment variant="chip" name="cover.png" icon="image" />
        <Attachment variant="chip" name="notes.md" icon="file-text" />
      </AttachmentGroup>
    </Column>
  );

  // section: Content and actions
  const actions = (
    <Column gap={8}>
      <Attachment
        name="signed-contract.pdf"
        meta="PDF · 2.1 MB"
        icon="file-text"
        status="complete"
        onDownload={() => {}}
        onRemove={() => {}}
      >
        <Text style={{ fontSize: 11, color: tokens.mutedForeground }}>
          Shared with 3 people · signed copy
        </Text>
      </Attachment>
      <Attachment
        name="budget.xlsx"
        meta="XLSX · 96 KB"
        icon="table"
        action={
          <Button
            label="Preview"
            size="xsmall"
            variant="secondary"
            accessibilityLabel="Preview budget.xlsx"
            onPress={() => {}}
          />
        }
        style={{ maxWidth: 360 }}
      />
      <Attachment name="read-only-brief.pdf" meta="PDF · 520 KB" icon="file-text" downloadable style={{ maxWidth: 360 }} />
    </Column>
  );

  // section: Whole-card click
  const opened = signal("No file opened");
  const wholeCard = (
    <Column gap={8}>
      <Attachment
        name="design-mockups.png"
        meta="PNG · 1.8 MB"
        image={cover}
        status="complete"
        onPress={() => opened.set("Opened design-mockups.png")}
        style={{ maxWidth: 380 }}
      />
      <Status>{opened()}</Status>
    </Column>
  );

  // section: Remove and retry controls
  const visible = signal(true);
  const retryStatus = signal<AttachmentStatus>("error");
  const retryProgress = signal(0);
  let retryTimer: number | undefined;
  const retry = () => {
    retryStatus.set("uploading");
    retryProgress.set(18);
    if (retryTimer !== undefined) clearTimeout(retryTimer);
    retryTimer = setTimeout(() => {
      retryProgress.set(100);
      retryStatus.set("complete");
    }, 1200);
  };
  const controls = (
    <Column gap={8}>
      {visible() ? (
        <Attachment
          name="invoice-042.pdf"
          meta="PDF · 640 KB"
          icon="file-text"
          status={retryStatus()}
          progress={retryProgress()}
          onRetry={retry}
          onRemove={() => visible.set(false)}
          style={{ maxWidth: 380 }}
        />
      ) : (
        <Button label="Restore attachment" size="small" onPress={() => visible.set(true)} />
      )}
      <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
        Retry moves a failed upload back through uploading and complete; remove drops the row.
      </Text>
    </Column>
  );
  // section: end

  return (
    <Page>
      <Caption>
        Attachments are quiet bordered surfaces: a file or image preview, a clear name, a small
        metadata line, and only the actions that make sense for the current state.
      </Caption>
      <ExampleSection index={0} title="Usage">
        {usage}
      </ExampleSection>
      <ExampleSection index={1} title="Anatomy and basic usage">
        {anatomy}
      </ExampleSection>
      <ExampleSection index={2} title="Lifecycle states">
        {states}
      </ExampleSection>
      <ExampleSection index={3} title="Upload queue across all states">
        {uploadQueue}
      </ExampleSection>
      <ExampleSection index={4} title="Media and image previews">
        {media}
      </ExampleSection>
      <ExampleSection index={5} title="Sizes and axes">
        {sizes}
      </ExampleSection>
      <ExampleSection index={6} title="Composer tray">
        {composer}
      </ExampleSection>
      <ExampleSection index={7} title="File picker">
        {picker}
      </ExampleSection>
      <ExampleSection index={8} title="Groups">
        {groups}
      </ExampleSection>
      <ExampleSection index={9} title="Content and actions">
        {actions}
      </ExampleSection>
      <ExampleSection index={10} title="Whole-card click">
        {wholeCard}
      </ExampleSection>
      <ExampleSection index={11} title="Remove and retry controls">
        {controls}
      </ExampleSection>
    </Page>
  );
}
