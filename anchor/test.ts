import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// import { Test } from "../target/types/test";
import * as borsh from "borsh";
import base58 from "bs58";


// describe("test", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   // const program = anchor.workspace.test as Program<Test>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });

class GreetingAccount {
  counter: number;
  message: string;
  is_active: boolean;

  constructor(fields: {
    counter: number;
    message: string;
    is_active: boolean;
  }) {
    Object.assign(this, fields);
  }
}

function de_serialize(){

  // 创建 Borsh 模式
  const GreetingAccountSchema = new Map([
    [
      GreetingAccount,
      {
        kind: "struct",
        fields: [
          ["counter", "u32"],
          ["message", "string"],
          ["is_active", "u8"], // bool 在 Borsh 中占 1 字节
        ],
      },
    ],
  ]);

  // 序列化数据
  const account = new GreetingAccount({
    counter: 10,
    message: "AABBCC",
    is_active: true,
  });

  const serializedData = borsh.serialize(GreetingAccountSchema, account);

  // const GREETING_SIZE = serializedData.length;

  console.log(Buffer.from(serializedData).toString("base64"));
  console.log(Buffer.from(serializedData));

  const str = base58.encode(Buffer.from(serializedData));
  console.log(str);
  // "data": "HE9J9tWuPZ3o7KEYZEGx"

}



///////
class InitData {
  // counter!: anchor.BN;     // u64 -> BN
  counter!: number;     // u64 -> BN
  message!: string; // string
  isActive!: number; // u8
  constructor(props: {counter: anchor.BN; message: string; isActive: number}) {
    Object.assign(this, props);
  }
}

// const schema = new Map([
//   [GreetingAccount, { kind: "struct", fields: [
//     ["counter", "u64"],
//     ["message", "string"],
//     ["isActive", "u8"],
//   ]}],
// ]);
const GreetingAccountSchema = new Map([
  [
    GreetingAccount,
    {
      kind: "struct",
      fields: [
        ["counter", "u32"],
        ["message", "string"],
        ["is_active", "u8"], // bool 在 Borsh 中占 1 字节
      ],
    },
  ],
]);


function decodeTransationData(){

  const dataBase58 = "11117UPu9wp46MgALqH1LfU6YF5U28rqqicymTkMQhrmrctMDR8Vie5izHQGfmBhaGLaHL";
  const data = base58.decode(dataBase58);

  // 第一步：前 8 个字节是 discriminator
  const discriminator = data.slice(0, 8);
  console.log("Discriminator:", Buffer.from(discriminator).toString("hex"));

  const init_data = {
    counter: 10, // Anchor 用 BN 表示 u64
    message: "Hello Solana",
    isActive: true, // 注意 JS/TS 命名风格可以和 Rust 对应
  };

  const account = new GreetingAccount({
    counter: 10,
    message: "Hello Solana",
    is_active: true,
  });

  const serializedData = borsh.serialize(GreetingAccountSchema, account);
  console.log(base58.encode(serializedData))
  // 第二步：剩下的就是参数（用 borsh 解码）
  // const decoded = Program.coder.instruction.decode(Buffer.from(data));


  // const deserializer = borsh.deserialize(schema, InitData,data.slice(8))
  
  // console.log(deserializer)

}

// decodeTransationData()

// 指令数据data
// const data = base58.decode("uBzDiRibakHW5TiTsmT3kpHea2uWCkeeCkoEaEuCcgPw6");
// console.log(data)
// console.log(Uint8Array.from(data))
// console.log(base58.encode(Uint8Array.from(data)))

// import bs58 from "bs58";
// import { Buffer } from "buffer";

function gerCreatAccountInstructionData() {
  // 创建账户指令数据
  const encoded = "11118dvrwJCphRrFu7HZqi6FrcUF8stdXdzhJhisn3torkU42BNxY42Zm7JGyHqYYjC6nx";
  const buf = base58.decode(encoded);

  const instructionType = buf.readUInt32LE(0);
  const lamports = buf.readBigUInt64LE(4);
  const space = buf.readBigUInt64LE(12);
  const programId = buf.slice(20, 52); // 32 bytes

  console.log({ instructionType, lamports, space, programId });
  const addr = base58.encode(programId);
  console.log(addr);

}

function decodeSystemInstruction(_dataB58: string) {

// 其中 instruction_type 对应下表（来自 Solana 文档）：

// 枚举值	指令	说明
// 0	CreateAccount	创建新账户
// 1	Assign	分配 owner
// 2	Transfer	转账
// 3	CreateAccountWithSeed	用 seed 创建账户
// 8	Allocate	只分配空间
// 9	AllocateWithSeed	用 seed 分配空间
// 10	AssignWithSeed	用 seed 指定 owner

  // 示例：你的 innerInstructions.data
  //系统指令解析
  const dataB58 = "11117UPu9wp46MgALqH1LfU6YF5U28rqqicymTkMQhrmrctMDR8Vie5izHQGfmBhaGLaHL";

  const buf = Buffer.from(base58.decode(dataB58));

  // 读取前 4 字节 (LE) = 指令类型
  const instructionType = buf.readUInt32LE(0);

  if (instructionType === 0) {
    // CreateAccount
    const lamports = buf.readBigUInt64LE(4);
    const space = buf.readBigUInt64LE(12);
    const owner_programId = base58.encode(buf.slice(20, 52));

    return {
      instruction: "CreateAccount",
      lamports: lamports.toString(),
      space: space.toString(),
      owner: owner_programId,
    };
  }

}
decodeSystemInstruction('');


