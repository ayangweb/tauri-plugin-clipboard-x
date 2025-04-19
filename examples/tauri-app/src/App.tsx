import { convertFileSrc } from "@tauri-apps/api/core";
import { Image, Table, TableProps } from "antd";
import { filesize } from "filesize";
import { useEffect, useState } from "react";
import {
  startListening,
  onClipboardChange,
  ReadClipboardItem,
} from "tauri-plugin-clipboard-x-api";

const App = () => {
  const [data, setData] = useState<ReadClipboardItem[]>();

  useEffect(() => {
    startListening();

    onClipboardChange((result) => {
      setData(Object.values(result));
    });
  }, []);

  const columns: TableProps<ReadClipboardItem>["columns"] = [
    {
      title: "类型",
      dataIndex: "type",
      key: "type",
      render: (type) => {
        switch (type) {
          case "text":
            return "纯文本";
          case "rtf":
            return "富文本";
          case "html":
            return "HTML";
          case "image":
            return "图像";
          case "files":
            return "文件";
        }
      },
    },
    {
      title: "内容",
      dataIndex: "value",
      key: "value",
      render: (value, item) => {
        const { type } = item;

        switch (type) {
          case "files":
            return `复制了${value.length}个文件`;
          case "image":
            return <Image width={100} src={convertFileSrc(value)} />;
          case "html":
            return <div dangerouslySetInnerHTML={{ __html: value }} />;
          default:
            return value;
        }
      },
    },
    {
      title: "大小",
      dataIndex: "count",
      key: "count",
      render: (count, item) => {
        const { type } = item;

        switch (type) {
          case "image":
          case "files":
            return filesize(count, { standard: "jedec" });
          default:
            return `${count}个字符`;
        }
      },
    },
    {
      title: "图像宽度",
      dataIndex: "width",
      key: "count",
      render: (width) => {
        return width ?? "-";
      },
    },
    {
      title: "图像高度",
      dataIndex: "height",
      key: "count",
      render: (height) => {
        return height ?? "-";
      },
    },
  ];

  return (
    <Table
      rowKey="type"
      columns={columns}
      dataSource={data}
      pagination={false}
    />
  );
};

export default App;
