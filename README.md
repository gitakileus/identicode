<div align="center">
	<h1>identicode</h1>
	<p><i>code that identifies you</i></p>
	<img src="https://img.shields.io/github/license/gulje/identicode?style=flat-square"/>
</div>

## Build & Installation
It can be downloaded and installed simply with the following command:
```sh
cargo install identicode
```

## How to use?
```
$	language mode
@	branch mode
?	os mode

+	add 1
*	add 5
/	add 10
%	add 50
=	add 100
-	substract 1

;	push language/branch/os by current value in stack 
```
When you use `;`, the string corresponding to the value in the stack is added whatever mode you are in.

For example, you want to add the Rust `(index: 203)` language.
- First, we need to switch to language mode, because we want to add a language: `$`
- Write the code corresponding to the index: `==+++`
- After you write the code, you can add Rust to the languages section by using `;`.
- Final code: `$==+++;`

You can also apply this process with operating systems (`?`) and branches (`@`).

## Indexes:
If you can't find what you want, you can create a [Pull Request](https://github.com/gulje/identicode/pulls)!
All lists are present in `src/lib.rs` file. Add what you want to add at the bottom,
and don't forget to add it to the [Indexes](#indexes) section in [README.md](README.md).

<details>
	<summary>Languages</summary>
	
```sh
0	4th Dimension/4D
1	ABAP
2	ABC
3	ActionScript
4	Ada
5	Agilent VEE
6	Algol
7	Alice
8	Angelscript
9	Apex
10	APL
11	AppleScript
12	Arc
13	Arduino
14	ASP
15	AspectJ
16	Assembly
17	ATLAS
18	Augeas
19	AutoHotkey
20	AutoIt
21	AutoLISP
22	Automator
23	Avenue
24	Awk
25	Bash
26	(Visual) Basic
27	bc
28	BCPL
29	BETA
30	BlitzMax
31	Boo
32	Bourne Shell
33	Bro
34	C
35	C Shell
36	C#
37	C++
38	C++/CLI
39	C-Omega
40	Caml
41	Ceylon
42	CFML
43	cg
44	Ch
45	CHILL
46	CIL
47	CL (OS/400)
48	Clarion
49	Clean
50	Clipper
51	Clojure
52	CLU
53	COBOL
54	Cobra
55	CoffeeScript
56	ColdFusion
57	COMAL
58	Common Lisp
59	Coq
60	cT
61	Curl
62	D
63	Dart
64	DCL
65	DCPU-16 ASM
66	Delphi/Object Pascal
67	DiBOL
68	Dylan
69	E
70	eC
71	Ecl
72	ECMAScript
73	EGL
74	Eiffel
75	Elixir
76	Emacs Lisp
77	Erlang
78	Etoys
79	Euphoria
80	EXEC
81	F#
82	Factor
83	Falcon
84	Fancy
85	Fantom
86	Felix
87	Forth
88	Fortran
89	Fortress
90	(Visual) FoxPro
91	Gambas
92	GNU Octave
93	Go
94	Google AppsScript
95	Gosu
96	Groovy
97	Haskell
98	haXe
99	Heron
100	HPL
101	HyperTalk
102	Icon
103	IDL
104	Inform
105	Informix-4GL
106	INTERCAL
107	Io
108	Ioke
109	J
110	J#
111	JADE
112	Java
113	Java FX Script
114	JavaScript
115	JScript
116	JScript.NET
117	Julia
118	Korn Shell
119	Kotlin
120	LabVIEW
121	Ladder Logic
122	Lasso
123	Limbo
124	Lingo
125	Lisp
126	Logo
127	Logtalk
128	LotusScript
129	LPC
130	Lua
131	Lustre
132	M4
133	MAD
134	Magic
135	Magik
136	Malbolge
137	MANTIS
138	Maple
139	Mathematica
140	MATLAB
141	Max/MSP
142	MAXScript
143	MEL
144	Mercury
145	Mirah
146	Miva
147	ML
148	Monkey
149	Modula-2
150	Modula-3
151	MOO
152	Moto
153	MS-DOS Batch
154	MUMPS
155	NATURAL
156	Nemerle
157	Nimrod
158	NQC
159	NSIS
160	Nu
161	NXT-G
162	Oberon
163	Object Rexx
164	Objective-C
165	Objective-J
166	OCaml
167	Occam
168	ooc
169	Opa
170	OpenCL
171	OpenEdge ABL
172	OPL
173	Oz
174	Paradox
175	Parrot
176	Pascal
177	Perl
178	PHP
179	Pike
180	PILOT
181	PL/I
182	PL/SQL
183	Pliant
184	PostScript
185	POV-Ray
186	PowerBasic
187	PowerScript
188	PowerShell
189	Processing
190	Prolog
191	Puppet
192	Pure Data
193	Python
194	Q
195	R
196	Racket
197	REALBasic
198	REBOL
199	Revolution
200	REXX
201	RPG (OS/400)
202	Ruby
203	Rust
204	S
205	S-PLUS
206	SAS
207	Sather
208	Scala
209	Scheme
210	Scilab
211	Scratch
212	sed
213	Seed7
214	Self
215	Shell
216	SIGNAL
217	Simula
218	Simulink
219	Slate
220	Smalltalk
221	Smarty
222	SPARK
223	SPSS
224	SQR
225	Squeak
226	Squirrel
227	Standard ML
228	Suneido
229	SuperCollider
230	TACL
231	Tcl
232	Tex
233	thinBasic
234	TOM
235	Transact-SQL
236	Turing
237	TypeScript
238	Vala/Genie
239	VBScript
240	Verilog
241	VHDL
242	VimL
243	Visual Basic .NET
244	WebDNA
245	Whitespace
246	X10
247	xBase
248	XBase++
249	Xen
250	XPL
251	XSLT
252	XQuery
253	yacc
254	Yorick
255	Z shell
```
</details>
<details>
	<summary>Branches</summary>

```
0	Human-computer interaction
1	Data science
2	Natural language processing
3	Programming languages
4	Software engineering
5	Architecture and organization
6	Cyber security
7	Information management
8	Networking and communication
9	Computer graphics
10	Platform-based development
11	Graphics and visual computing
12	Algorithms and complexity
13	Parallel and distributed computing
14	Intelligent systems
15	Security and information assurance
16	Computer Science
17	Computer Engineering
18	Information Systems
19	New Media
20	Information Technology (IT)
21	Information Science
22	Mathematical foundations
23	Algorithms and data structures
24	Artificial intelligence
25	Communication and security
26	Computer architecture
27	Computer graphics
28	Concurrent, parallel, and distributed systems
29	Databases
30	Programming languages and compilers
31	Scientific computing
32	Software engineering
33	Theory of computing
```
</details>
<details>
	<summary>Operating Systems</summary>
	
```
0	Arthur
1	RISC OS
2	Fire OS
3	Amiga OS
4	AMSDOS
5	macOS
6	iOS
7	iPadOS
8	tvOS
9	bridgeOS
10	Atari DOS
11	BeOS
12	Unix
13	BESYS
14	Plan 9
15	Inferno
16	Android
17	Harmony OS
18	LiteOS
19	iRMX
20	PC DOS
21	OS/2
22	Remix OS
23	KaiOS
24	LynxOS
25	Xenix
26	MS-DOS
27	DOS/V
28	Windows
29	Windows 1.0
30	Windows 2.0
31	Windows 3.0
32	Windows 3.1x
33	Windows 3.2
34	Windows 95
35	Windows 98
36	Windows ME
37	Windows NT
38	Windows NT 3.1
39	Windows NT 4.0
40	Windows 2000
41	Windows XP
42	Windows Server 2003
43	Windows Vista
44	Windows Phone 7
45	Windows 8
46	Windows RT
47	Windows Phone 8
48	Windows 8.1
49	Windows Phone 8.1
50	Windows 10
51	Windows 10 Mobile
52	Windows 11
53	ES
54	NeXTSTEP
55	NetWare
56	UnixWare
57	Bada
58	Tizen
59	One UI
60	Sun OS
61	BSD
62	FreeBSD
63	DragonFlyBSD
64	MidnightBSD
65	GhostBSD
66	TrueOS
67	NetBSD
68	OpenBSD
69	Bitrig
70	Darwin
71	GNU Hurd
72	Linux
73	RHEL
74	Rocky Linux
75	Red Hat Linux
76	CentOS
77	Fedora
78	openSUSE
79	SUSE Linux Enterprise Desktop
80	SUSE Linux Enterprise Server
81	SUSE Studio
82	GeckoLinux
83	Mandrake Linux
84	Debian
85	MX Linux
86	Deepin
87	Devuan
88	Kali Linux
89	Pure OS
90	Ubuntu
91	Kubuntu
92	Lubuntu
93	Ubuntu Budgie
94	Ubuntu Kylin
95	Ubuntu Mate
96	Xubuntu
97	Bodhi Linux
98	elementary OS
99	Linux Mint
100	Zorin OS
101	Pop!_OS
102	Arch Linux
103	Manjaro
104	Artix Linux
105	EndeavourOS
106	SteamOS
107	Gentoo
108	Chrome OS
109	Chromium OS
110	NixOS
111	Void Linux
112	GuixSD
113	Solus
114	Redox
115	illumos
116	OpenIndiana
117	FreeDOS
118	Genode
119	FFusionOS
120	Ghost OS
121	Haiku
122	ReactOS
123	TempleOS
124	Serenity
125	Visopsys
```
</details>

## License
`identicode` is licensed under the terms of [Apache License 2.0](LICENSE).
