
Î
appleØ
single_arch_cpustring"ìThe single "effective" architecture for this configuration (e.g., <code>i386</code> or <code>arm64</code>) in the context of rule logic that is only concerned with a single architecture (such as <code>objc_library</code>, which registers single-architecture compile actions).Ä
single_arch_platformapple_platform"◊The platform of the current configuration. This should only be invoked in a context where only a single architecture may be supported; consider <a href='#multi_arch_platform'>multi_arch_platform</a> for other cases.-A configuration fragment for Apple platforms.
t
bazel_androidc
"merge_android_manifest_permissionsbool"7The value of --merge_android_manifest_permissions flag.
µ
bazel_pyl
python_import_all_repositoriesbool"DThe value of the --experimental_python_import_all_repositories flag.;
python_pathstring"$The value of the --python_path flag.
Ú
coverage¢
output_generatorLabel"ÜReturns the label pointed to by the <a href="https://bazel.build/reference/command-line-reference#flag--coverage_output_generator"><code>--coverage_output_generator</code></a> option if coverage collection is enabled, otherwise returns <code>None</code>. Can be accessed with <a href="../globals/bzl.html#configuration_field"><code>configuration_field</code></a>:<br/><pre>attr.label(<br/>    default = configuration_field(<br/>        fragment = "coverage",<br/>        name = "output_generator"<br/>    )<br/>)</pre>AA configuration fragment representing the coverage configuration.
≈

cppU
apple_generate_dsymbool"8Whether to generate Apple debug symbol(.dSYM) artifacts.~
	conlyoptslist"kThe flags passed to Bazel by <a href="/docs/user-manual#flag--conlyopt"><code>--conlyopt</code></a> option.r
coptslist"cThe flags passed to Bazel by <a href="/docs/user-manual#flag--copt"><code>--copt</code></a> option.ç
custom_mallocLabel"ÙReturns label pointed to by <a href="/docs/user-manual#flag--custom_malloc"><code>--custom_malloc</code></a> option. Can be accessed with <a href="../globals/bzl.html#configuration_field"><code>configuration_field</code></a>:<br/><pre>attr.label(<br/>    default = configuration_field(<br/>        fragment = "cpp",<br/>        name = "custom_malloc"<br/>    )<br/>)</pre>x
cxxoptslist"gThe flags passed to Bazel by <a href="/docs/user-manual#flag--cxxopt"><code>--cxxopt</code></a> option.{
linkoptslist"iThe flags passed to Bazel by <a href="/docs/user-manual#flag--linkopt"><code>--linkopt</code></a> option.R
objc_generate_linkmapbool"3(Apple-only) Whether to generate linkmap artifacts.u
objc_should_strip_binarybool"S(Apple-only) whether to perform symbol and dead-code strippings on linked binaries.~
	objccoptslist"kThe flags passed to Bazel by <a href="/docs/user-manual#flag--objccopt"><code>--objccopt</code></a> option.!A configuration fragment for C++.
ç
j2objc]
translation_flagslist"BThe list of flags to be used when the j2objc compiler is invoked. $A configuration fragment for j2Objc.
„
javaÙ
"bytecode_optimization_pass_actionsint"»This specifies the number of actions to divide the OPTIMIZATION stage of the bytecode optimizer into. Note that if split_bytecode_optimization_pass is set, this will only change behavior if it is > 2.O
bytecode_optimizer_mnemonicstring"(The mnemonic for the bytecode optimizer.E
default_javac_flagslist"(The default flags for the Java compiler.N
default_javac_flags_depsetdepset"(The default flags for the Java compiler._
default_jvm_optslist"EAdditional options to pass to the Java VM for each java_binary target\
disallow_java_import_exportsbool"4Returns true if java_import exports are not allowed.b
multi_release_deploy_jarsbool"?The value of the --incompatible_multi_release_deploy_jars flag.f
one_version_enforcement_levelstring"=The value of the --experimental_one_version_enforcement flag.N
pluginslist"=A list containing the labels provided with --plugins, if any.^
run_android_lintbool"DThe value of the --experimental_run_android_lint_on_java_rules flag.å
 split_bytecode_optimization_passbool"bReturns whether the OPTIMIZATION stage of the bytecode optimizer will be split across two actions.C
strict_java_depsstring"'The value of the strict_java_deps flag.Ç
"use_header_compilation_direct_depsbool"TReturns true if Java header compilation should use separate outputs for direct deps.H
	use_ijarsbool"3Returns true iff Java compilation should use ijars.A java compiler configuration.
˚	
objcn
alwayslink_by_defaultbool"OReturns whether objc_library and objc_import should default to alwayslink=True.b
builtin_objc_strip_actionbool"?Returns whether to emit a strip action as part of objc linking.É
"copts_for_current_compilation_modelist"WReturns a list of default options to use for compiling Objective-C in the current mode.}
"disallow_sdk_frameworks_attributesbool"QReturns whether sdk_frameworks and weak_sdk_frameworks are disallowed attributes.j
ios_simulator_devicestring"JThe type of device (e.g. 'iPhone 6') to use when running on the simulator.r
ios_simulator_versionDottedVersion"JThe SDK version of the iOS simulator to use when running on the simulator.f
run_memleaksbool"PReturns a boolean indicating whether memleaks should be run during tests or not.ì
signing_certificate_namestring"oReturns the flag-supplied certificate name to be used in signing, or None if no such certificate was specified.å
strip_executable_safelybool"kReturns whether executable strip action should use flag -x, which does not break dynamic symbol resolution.Å
uses_device_debug_entitlementsbool"YReturns whether device debug entitlements should be included when signing an application.)A configuration fragment for Objective-C.
ä
platform1
host_platformLabel"The current host platform.
platformLabel"The current target platformThe platform configuration.
@
proto7A configuration fragment representing protocol buffers.
À
pyC
build_python_zipbool")The effective value of --build_python_zipK
default_python_versionstring")No-op: PY3 is the default Python version.g
default_to_explicit_init_pybool"BThe value from the --incompatible_default_to_explicit_init_py flaga
disallow_native_rulesbool"BThe value of the --incompatible_python_disallow_native_rules flag.A
use_toolchainsbool")No-op: Python toolchains are always used.$A configuration fragment for Python.
ö
AnalysisTestResultInfog
messagestring"TA descriptive message containing information about the test and its success/failure.t
successbool"cIf true, then the analysis-phase test represented by this target passed. If false, the test failed.†Encapsulates the result of analyis-phase testing. Build targets which return an instance of this provider signal to the build system that it should generate a 'stub' test executable which generates the equivalent test result. Analysis test rules (rules created with <code>analysis_test=True</code> <b>must</b> return an instance of this provider, and non-analysis-phase test rules <b>cannot</b> return this provider.
±
CcInfoV
compilation_contextCompilationContext"+Returns the <code>CompilationContext</code>B
linking_contextstruct"'Returns the <code>LinkingContext</code>äA provider for compilation and linking of C++. This is also a marking provider telling C++ rules that they can depend on the rule with this provider. If it is not intended for the rule to be depended on by C++, the rule should wrap the CcInfo in some other provider.
Ø
CcToolchainConfigInfoïAdditional layer of configurability for C++ rules. Encapsulates platform-dependent specifics of C++ actions through features and action configs. It is used to configure the C++ toolchain, and later on for command line construction. Replaces the functionality of CROSSTOOL file.
’
CcToolchainInfou
	all_filesNoneType"^Returns all toolchain files (so they can be passed to actions using this toolchain as inputs).5
ar_executableNoneType"The path to the ar binary.c
built_in_include_directoriesNoneType"9Returns the list of built-in directories of the compiler.#
compilerNoneType"C++ compiler.A
compiler_executableNoneType" The path to the compiler binary.1
cpuNoneType" Target CPU of the C++ toolchain.—
dynamic_runtime_libK
?
feature_configuration$Feature configuration to be queried.(NoneType"ÏReturns the files from `dynamic_runtime_lib` attribute (so they can be passed to actions using this toolchain as inputs). The caller can check whether the feature_configuration enables `static_link_cpp_runtimes` feature (if not, neither `static_runtime_lib` nor `dynamic_runtime_lib` have to be used), and use `static_runtime_lib` if static linking mode is active.9
gcov_executableNoneType"The path to the gcov binary.5
ld_executableNoneType"The path to the ld binary.&
libcNoneType"libc version string.Œ
needs_pic_for_dynamic_librariesK
?
feature_configuration$Feature configuration to be queried.(NoneType"›Returns true if this rule's compilations should apply -fPIC, false otherwise. Determines if we should apply -fPIC for this rule's C++ compilations depending on the C++ toolchain and presence of `--force_pic` Bazel option.5
nm_executableNoneType"The path to the nm binary.?
objcopy_executableNoneType"The path to the objcopy binary.?
objdump_executableNoneType"The path to the objdump binary.I
preprocessor_executableNoneType"$The path to the preprocessor binary.”
static_runtime_libK
?
feature_configuration$Feature configuration to be queried.(NoneType"ÔReturns the files from `static_runtime_lib` attribute (so they can be passed to actions using this toolchain as inputs). The caller should check whether the feature_configuration enables `static_link_cpp_runtimes` feature (if not, neither `static_runtime_lib` nor `dynamic_runtime_lib` should be used), and use `dynamic_runtime_lib` if dynamic linking mode is active.;
strip_executableNoneType"The path to the strip binary.”
sysrootNoneType"ΩReturns the sysroot to be used. If the toolchain compiler does not support different sysroots, or the sysroot is the same as the default sysroot, then this method returns <code>None</code>.8
target_gnu_system_nameNoneType"The GNU System Name..Information about the C++ compiler being used.
â
ConstraintCollectionProvides access to data about a collection of ConstraintValueInfo providers. <br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>
◊
ConstraintSettingInfoc
has_default_constraint_valuebool"=Whether there is a default constraint_value for this setting.ÿA specific constraint setting that may be used to define a platform. See <a href='/docs/platforms#constraints-platforms'>Defining Constraints and Platforms</a> for more information.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>
Û
ConstraintValueInfo€A value for a constraint setting that can be used to define a platform. See <a href='/docs/platforms#constraints-platforms'>Defining Constraints and Platforms</a> for more information.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>
ë
DebugPackageInfoU
dwp_fileFile"CReturns the .dwp file (for fission builds) or null if --fission=no.S
stripped_fileFile"<Returns the stripped file (the explicit ".stripped" target).@
target_labelLabel")Returns the label for the *_binary targetT
unstripped_fileFile";Returns the unstripped file (the default executable target)∏A provider for the binary file and its associated .dwp files, if fission is enabled.If Fission ({@url https://gcc.gnu.org/wiki/DebugFission}) is not enabled, the dwp file will be null.
„

DefaultInfo˘
data_runfilesrunfiles"›runfiles descriptor describing the files that this target needs when run in the condition that it is a <code>data</code> dependency attribute. Under most circumstances, use the <code>default_runfiles</code> parameter instead. See <a href='https://bazel.build/extending/rules#runfiles_features_to_avoid'>"runfiles features to avoid"</a> for details. £
default_runfilesrunfiles"Ñrunfiles descriptor describing the files that this target needs when run (via the <code>run</code> command or as a tool dependency).õ
filesdepset"âA <a href='../builtins/depset.html'><code>depset</code></a> of <a href='../builtins/File.html'><code>File</code></a> objects representing the default outputs to build when this target is specified on the bazel command line. By default it is all predeclared outputs.»
files_to_runFilesToRunProvider"£A <a href='../providers/FilesToRunProvider.html'><code>FilesToRunProvider</code></a> object containing information about the executable and runfiles of the target.»A provider that gives general information about a target's direct and transitive files. Every rule type has this provider, even if it is not returned explicitly by the rule's implementation function.
<p>
See the <a href="https://bazel.build/extending/rules">rules</a> page for extensive guides on how to use this provider.
</p>

ú
ExecutionInfoR

exec_groupstring"<The name of the exec group that is used to execute the test.c
requirementsdict"MA dict indicating special execution requirements, such as hardware platforms.RUse this provider to specify special environment requirements needed to run tests.
ä
FeatureFlagInfod
errorstring"SIf non-None, this error was generated when trying to compute current value of flag.ÿ
is_valid_valueÇ
z
value7<a class="anchor" href="../core/string.html">string</a>6String, the value to check for validity for this flag.(bool"AThe value of the flag in the configuration used by the flag rule.n
valuestring"]The current value of the flag in the flag's current configuration. None if there is an error.FA provider used to access information about config_feature_flag rules.
;
file_provider*An interface for rules that provide files.
´
FilesToRunProviderE

executableFile"1The main executable or None if it does not exist.V
repo_mapping_manifestFile"7The repo mapping manifest or None if it does not exist.N
runfiles_manifestFile"3The runfiles manifest or None if it does not exist.•Contains information about executables produced by a target and the files needed to run it. This provider can not be created directly, it is an implicit output of executable targets accessible via <a href="../providers/DefaultInfo.html#files_to_run"><code>DefaultInfo.files_to_run</code></a>.

ê
IncompatiblePlatformProviderÔA provider for targets that are incompatible with the target platform. See <a href='/docs/platforms#detecting-incompatible-targets-using-bazel-cquery'>Detecting incompatible targets using <code>bazel cquery</code></a> for more information.
∏
InstrumentedFilesInfoÊ
instrumented_filesdepset"«<a href="../builtins/depset.html"><code>depset</code></a> of <a href="../builtins/File.html"><code>File</code></a> objects representing instrumented source files for this target and its dependencies.ƒ
metadata_filesdepset"©<a href="../builtins/depset.html"><code>depset</code></a> of <a href="../builtins/File.html"><code>File</code></a> objects representing coverage metadata files for this target and its dependencies. These files contain additional information required to generate LCOV-format coverage output after the code is executed, e.g. the <code>.gcno</code> files generated when <code>gcc</code> is run with <code>-ftest-coverage</code>.ÓContains information about source files and instrumentation metadata files for rule targets matched by <a href="https://bazel.build/reference/command-line-reference#flag--instrumentation_filter"><code>--instrumentation_filter</code></a> for purposes of <a href="https://bazel.build/extending/rules#code_coverage">code coverage data collection</a>. When coverage data collection is enabled, a manifest containing the combined paths in <a href="#instrumented_files"><code>instrumented_files</code></a> and <a href="#metadata_files"><code>metadata_files</code></a> are passed to the test action as inputs, with the manifest's path noted in the environment variable <code>COVERAGE_MANIFEST</code>. The metadata files, but not the source files, are also passed to the test action as inputs. When <code>InstrumentedFilesInfo</code> is returned by an <a href="https://bazel.build/extending/aspects">aspect</a>'s implementation function, any <code>InstrumentedFilesInfo</code> from the base rule target is ignored.
‡
java_compilation_info<
boot_classpathlist"$Boot classpath for this Java target.L
compilation_classpathdepset"+Compilation classpath for this Java target.∑
javac_optionsdepset"ùA depset of options to java compiler. To get the exact list of options passed to javac in the correct order, use the tokenize_javacopts utility in rules_javaE
runtime_classpathdepset"(Run-time classpath for this Java target.:Provides access to compilation information for Java rules.
‚
java_output_jarsv
jarslist"hReturns information about outputs of this Java/Java-like target. Deprecated: Use java_info.java_outputs.¸
jdepsFile"ÏA manifest proto file. The protobuf file containing the manifest generated from JavaBuilder. This function returns a value when exactly one manifest proto file is present in the outputs.  Deprecated: Use java_info.java_outputs[i].jdeps.É
native_headersFile"ÍA jar containing CC header files supporting native method implementation.  This function returns a value when exactly one native headers jar file is present in the outputs. Deprecated: Use java_info.java_outputs[i].native_headers_jar.QInformation about outputs of a Java rule. Deprecated: use java_info.java_outputs.
«

JavaRuntimeInfo9
default_cdsFile"$Returns the JDK default CDS archive.7
filesdepset"&Returns the files in the Java runtime.`
hermetic_filesdepset"FReturns the files in the Java runtime needed for hermetic deployments.C
hermetic_static_libssequence"!Returns the JDK static libraries.Q
java_executable_exec_pathstring",Returns the execpath of the Java executable.Õ
java_executable_runfiles_pathstring"£Returns the path of the Java executable in runfiles trees. This should only be used when one needs to access the JVM during the execution of a binary or a test built by Bazel. In particular, when one needs to invoke the JVM during an action, java_executable_exec_path should be used instead.O
	java_homestring":Returns the execpath of the root of the Java installation.Ø
java_home_runfiles_pathstring"ãReturns the path of the Java installation in runfiles trees. This should only be used when one needs to access the JDK during the execution of a binary or a test built by Bazel. In particular, when one needs the JDK during an action, java_home should be used instead.0

lib_ct_symFile"Returns the lib/ct.sym file.2
lib_modulesFile"Returns the lib/modules file.]
versionint"MThe Java feature version of the runtime. This is 0 if the version is unknown..Information about the Java runtime being used.
ï
JavaToolchainInfok
bootclasspathdepset"RThe Java target bootclasspath entries. Corresponds to javac's -bootclasspath flag.R
ijarFilesToRunProvider"6A FilesToRunProvider representing the ijar executable.K
jacocorunnerFilesToRunProvider"'The jacocorunner used by the toolchain.>
java_runtimeJavaRuntimeInfo"The java runtime information.b
jvm_optdepset"OThe default options for the JVM running the java compiler and associated tools.$
labelLabel"The toolchain label.`
proguard_allowlisterFilesToRunProvider"4Return the binary to validate proguard configuration;

single_jarFilesToRunProvider"The SingleJar deploy jar.2
source_versionstring"The java source version.2
target_versionstring"The java target version.'
toolsdepset"The compilation tools.xProvides access to information about the Java toolchain rule. Accessible as a 'java_toolchain' field on a Target struct.
«
MaterializedDepsInfoc
depslist"UThe list of dependencies. These may be ConfiguredTarget or DormantDependency objects.JThe provider returned from materializer rules to materialize dependencies.
»
ObjcProvider∂
direct_module_mapssequence"ïModule map files from this target directly (no transitive module maps). Used to enforce proper use of private header files and for Swift compilation.ä
direct_sourcessequence"nAll direct source files from this target (no transitive files), including any headers in the 'srcs' attribute.[
j2objc_librarydepset"AStatic libraries that are built from J2ObjC-translated Java code.\

module_mapdepset"FClang module maps, used to enforce proper use of private header files..
sourcedepset"All transitive source files.¿
strict_includedepset"•Non-propagated include search paths specified with '-I' on the command line. Also known as header search paths (and distinct from <em>user</em> header search paths).í
umbrella_headerdepset"wClang umbrella header. Public headers are #included in umbrella headers to be compatible with J2ObjC segmented headers./A provider for compilation and linking of objc.
À
OutputGroupInfo∑A provider that indicates what output groups a rule has.<br>See <a href="https://bazel.build/extending/rules#requesting_output_files">Requesting output files</a> for more information.
Ú
PackageSpecificationInfoà
containsÃ
√
targetu<a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../core/string.html">string</a>@A target which is checked if it exists inside the package group.(bool"-Checks if a target exists in a package group.KInformation about transitive package specifications used in package groups.
◊
PlatformInfo∆Provides access to data about a specific platform. See <a href='/docs/platforms#constraints-platforms'>Defining Constraints and Platforms</a> for more information.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>
¬
RunEnvironmentInfoÍ
environmentdict"‘A map of string keys and values that represent environment variables and their values. These will be made available when the target that returns this provider is executed, either as a test or via the run command.≈
inherited_environmentlist"•A sequence of names of environment variables. These variables are made available with their current value taken from the shell environment when the target that returns this provider is executed, either as a test or via the run command. If a variable is contained in both <code>environment</code> and <code>inherited_environment</code>, the value inherited from the shell environment will take precedence if set. This is most useful for test rules, which run with a hermetic environment under <code>bazel test</code> and can use this mechanism to non-hermetically include a variable from the outer environment. By contrast, <code>bazel run</code> already forwards the outer environment. Note, though, that it may be surprising for an otherwise hermetic test to hardcode a non-hermetic dependency on the environment, and that this may even accidentally expose sensitive information. Prefer setting the test environment explicitly with the <code>--test_env</code> flag, and even then prefer to avoid using this flag and instead populate the environment explicitly.wA provider that can be returned from executable rules to control the environment in which their executable is executed.
á
TemplateVariableInfow
	variablesdict"dReturns the make variables defined by this target as a dictionary with string keys and string valuesıEncapsulates template variables, that is, variables that can be referenced by strings like <code>$(VARIABLE)</code> in BUILD files and expanded by <code>ctx.expand_make_variables</code> and implicitly in certain attributes of built-in rules.</p><p><code>TemplateVariableInfo</code> can be created by calling its eponymous constructor with a string-to-string dict as an argument that specifies the variables provided.</p><p>Example: <code>platform_common.TemplateVariableInfo({'FOO': 'bar'})</code></p>
™
ToolchainInfoòProvider returned by <a href="/docs/toolchains#defining-toolchains">toolchain rules</a> to share data with <a href="/docs/toolchains#writing-rules-that-use-toolchains">rules which depend on toolchains</a>. Read about <a href='/docs/toolchains'>toolchains</a> for more information.
º
ToolchainTypeInfoH

type_labelLabel"3The label uniquely identifying this toolchain type.‹Provides access to data about a specific toolchain type. <br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>
¢
Action—
argssequence"æA list of frozen <a href="../builtins/Args.html">Args</a> objects containing information about the action arguments. These objects contain accurate argument information, including arguments involving expanded action output directories. However, <a href="../builtins/Args.html">Args</a> objects are not readable in the analysis phase. For a less accurate account of arguments which is available in the analysis phase, see <a href="#argv">argv</a>. <p>Note that some types of actions do not yet support exposure of this field. For such action types, this is <code>None</code>.›
argvsequence" For actions created by <a href="../builtins/actions.html#run">ctx.actions.run()</a> or <a href="../builtins/actions.html#run_shell">ctx.actions.run_shell()</a>  an immutable list of the arguments for the command line to be executed. Note that for shell actions the first two arguments will be the shell path and <code>"-c"</code>.Ó
contentstring"⁄For actions created by <a href="../builtins/actions.html#write">ctx.actions.write()</a> or <a href="../builtins/actions.html#expand_template">ctx.actions.expand_template()</a>, the contents of the file to be written, if those contents can be computed during  the analysis phase. The value is <code>None</code> if the contents cannot be determined until the execution phase, such as when a directory in an <a href="../builtins/Args.html">Args</a> object needs to be expanded.Ë
envdict"⁄The 'fixed' environment variables for this action. This includes only environment settings which are explicitly set by the action definition, and thus omits settings which are only pre-set in the execution environment.:
inputsdepset"(A set of the input files of this action.1
mnemonicstring"The mnemonic for this action.<
outputsdepset")A set of the output files of this action.∏
substitutionsdict"†For actions created by <a href="../builtins/actions.html#expand_template">ctx.actions.expand_template()</a>, an immutable dict holding the substitution mapping.üAn action created during rule analysis.<p>This object is visible for the purpose of testing, and may be obtained from an <code>Actions</code> provider. It is normally not necessary to access <code>Action</code> objects or their fields within a rule's implementation function. You may instead want to see the <a href='https://bazel.build/extending/rules#actions'>Rules page</a> for a general discussion of how to use actions when defining custom rules, or the <a href='../builtins/actions.html'>API reference</a> for creating actions.<p>Some fields of this object are only applicable for certain kinds of actions. Fields that are inapplicable are set to <code>None</code>.
à‡
actions`
argsArgs"PReturns an Args object that can be used to build memory-efficient command lines.â
declare_directoryº
ﬂ
filename7<a class="anchor" href="../core/string.html">string</a>óIf no 'sibling' provided, path of the new directory, relative to the current package. Otherwise a base name for a file ('sibling' defines a directory).(
—
siblingM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>qA file that lives in the same directory as the newly declared directory. The file must be in the current package."NoneFile"¥Declares that the rule or aspect creates a directory with the given name, in the current package. You must create an action that generates the directory. The contents of the directory are not directly accessible from Starlark, but can be expanded in an action command with <a href="../builtins/Args.html#add_all"><code>Args.add_all()</code></a>. Only regular files and directories can be in the expanded contents of a declare_directory.„

declare_file¥
›
filename7<a class="anchor" href="../core/string.html">string</a>ïIf no 'sibling' provided, path of the new file, relative to the current package. Otherwise a base name for a file ('sibling' determines a directory).(
À
siblingM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>kA file that lives in the same directory as the newly created file. The file must be in the current package."NoneFile"õDeclares that the rule or aspect creates a file with the given filename. If <code>sibling</code> is not specified, the file name is relative to the package directory, otherwise the file is in the same directory as <code>sibling</code>. Files cannot be created outside of the current package.<p>Remember that in addition to declaring a file, you must separately create an action that emits the file. Creating that action will require passing the returned <code>File</code> object to the action's construction function.<p>Note that <a href='https://bazel.build/extending/rules#files'>predeclared output files</a> do not need to be (and cannot be) declared using this function. You can obtain their <code>File</code> objects from <a href="../builtins/ctx.html#outputs"><code>ctx.outputs</code></a> instead. <a href="https://github.com/bazelbuild/examples/tree/main/rules/computed_dependencies/hash.bzl">See example of use</a>.·
declare_symlinkè
›
filename7<a class="anchor" href="../core/string.html">string</a>ïIf no 'sibling' provided, path of the new symlink, relative to the current package. Otherwise a base name for a file ('sibling' defines a directory).(
¶
siblingM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>FA file that lives in the same directory as the newly declared symlink."NoneFile"ªDeclares that the rule or aspect creates a symlink with the given name in the current package. You must create an action that generates this symlink. Bazel will never dereference this symlink and will transfer it verbatim to sandboxes or remote executors. Symlinks inside tree artifacts are not currently supported.ù

do_nothingâ
è
mnemonic7<a class="anchor" href="../core/string.html">string</a>HA one-word description of the action, for example, CppCompile or GoLink.(
Í
inputs≥<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <a class="anchor" href="../builtins/depset.html">depset</a>&List of the input files of the action."[]NoneType"ÇCreates an empty action that neither executes a command nor produces any output, but that is useful for inserting 'extra actions'.∞

expand_templateü
}
template7<a class="anchor" href="../builtins/File.html">File</a>6The template file, which is a UTF-8 encoded text file.(
y
output7<a class="anchor" href="../builtins/File.html">File</a>4The output file, which is a UTF-8 encoded text file.(
|
substitutions3<a class="anchor" href="../core/dict.html">dict</a>2Substitutions to make when expanding the template."{}
z
is_executable3<a class="anchor" href="../core/bool.html">bool</a>-Whether the output file should be executable."False
û
computed_substitutionsG<a class="anchor" href="../builtins/TemplateDict.html">TemplateDict</a>2Substitutions to make when expanding the template."unboundNoneType"˙Creates a template expansion action. When the action is executed, it will generate a file based on a template. Parts of the template will be replaced using the <code>substitutions</code> dictionary, in the order the substitutions are specified. Whenever a key of the dictionary appears in the template (or a result of a previous substitution), it is replaced with the associated value. There is no special syntax for the keys. You may, for example, use curly braces to avoid conflicts (for example, <code>{KEY}</code>). <a href="https://github.com/bazelbuild/examples/blob/main/rules/expand_template/hello.bzl">See example of use</a>.‡2
map_directory—1
á
input_directorieso<a class="anchor" href="../core/dict.html">dict</a> of <a class="anchor" href="../builtins/File.html">File</a>s˛A dictionary mapping of strings to input directories, as declared by <code>ctx.actions.declare_directory()</code> (only directories are allowed as values here). The values specify the directories that we want expanded to access their files in the implementation function. The keys (strings) act as identifiers to easily reference a specific directory in the implementation function.(
∫
additional_inputs3<a class="anchor" href="../core/dict.html">dict</a>ÎA dictionary of mapping of strings to additional inputs (only files, FilesToRunProvider(s) and Depset(s) are allowed here). The values specify any additional inputs that we want to make accessible to actions created by the implementation function. The keys (strings) act as identifiers to easily reference a specific input from within the implementation function."{}
ˆ
output_directorieso<a class="anchor" href="../core/dict.html">dict</a> of <a class="anchor" href="../builtins/File.html">File</a>sÏA dictionary mapping of strings to output directories, as declared by <code>ctx.actions.declare_directory()</code>. The values specify the output directories that we want to generate by the actions created by the implementation function. The keys (strings) act as identifiers to easily reference a specific output directory from within the implementation function.(
ê
tools3<a class="anchor" href="../core/dict.html">dict</a>œA dictionary mapping of strings to tools (only files, FilesToRunProvider(s) and Depset(s) are allowed here). The values specify the tools that we want to make accessible to actions created by the implementation function. The keys (strings) act as identifiers to easily reference a specific tool from within the implementation function.(
’
additional_params3<a class="anchor" href="../core/dict.html">dict</a>ÜA dictionary mapping of strings to additional parameters (only string, boolean and integer values are allowed here). The values specify any additional parameters that we want to make accessible to the implementation function that could be used to influence its behavior. The keys (strings) act as identifiers to easily reference a specific parameter from within the implementation function."{}
œ
execution_requirementsI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>dInformation for scheduling the created actions. See <a href="#common.tags">tags</a> for useful keys."None
ﬂ

exec_groupM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>|Run the created actions on the given exec group's execution platform. If none, uses the target's default execution platform."None
æ
	toolchainã<a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>ú<p>Toolchain type of the executable or tools used by the created actions.</p><p>If executable and tools are not coming from a toolchain, set this parameter to <code>None</code>.</p><p>If executable and tools are coming from a toolchain, toolchain type must be set so that the created actions execute on the correct execution platform.</p><p>Note that the rule which creates these actions needs to define this toolchain inside its 'rule()' function.</p><p>When <code>toolchain</code> and <code>exec_group</code> parameters are both set, <code>exec_group</code> will be used. An error is raised in case the <code>exec_group</code> doesn't specify the same toolchain.</p>"None

use_default_shell_env3<a class="anchor" href="../core/bool.html">bool</a>öWhether the created actions should use the default shell environment, which consists of a few OS-dependent variables as well as variables set via <a href="/reference/command-line-reference#flag--action_env"><code>--action_env</code></a>.<p>If both <code>use_default_shell_env</code> and <code>env</code> are set to <code>True</code>, values set in <code>env</code> will overwrite the default shell environment."False
≥
envI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>⁄Sets the dictionary of environment variables.<p>If both <code>use_default_shell_env</code> and <code>env</code> are set to <code>True</code>, values set in <code>env</code> will overwrite the default shell environment."None
≤
mnemonicM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>QA one-word description of the created actions, for example, CppCompile or GoLink."None
∆
implementation;<a class="anchor" href="../core/function.html">function</a>ÙA Starlark function that gets called after input directories have been built to generate actions
that output files to the specified output directories. This function is passed the following
arguments:

<ul>
  <li><code>template_ctx</code> (positional): A <a
      href='../builtins/template_ctx.html'><code>template_ctx</code></a> object that can be used to
      create actions.</li>
  <li><code>input_directories</code> (keyword-only): A dictionary mapping from the string keys of
      the <code>input_directories</code> argument of <code>actions.map_directory()</code> to their
      values' corresponding <a href='../builtins/File.html'><code>ExpandedDirectory</code></a>
      objects.</li>
  <li><code>output_directories</code> (keyword-only): The value of the
      <code>output_directories</code> argument of <code>actions.map_directory()</code>; a
      dictionary mapping from strings to output directories.</li>
  <li><code>additional_inputs</code> (keyword-only): The value of the
      <code>additional_inputs</code> argument of <code>actions.map_directory()</code>; a
      dictionary mapping from strings to input files.</li>
  <li><code>tools</code> (keyword-only): The value of the <code>tools</code> argument of
      <code>actions.map_directory()</code>; a dictionary mapping from strings to tools.</li>
  <li><code>additional_params</code> (keyword-only): The value of the
      <code>additional_params</code> argument of <code>actions.map_directory()</code>; a
      dictionary mapping from strings to strings, booleans, or integers.</li>
</ul>

This function must be top-level, i.e. lambdas and nested functions are not allowed.
(NoneType"{Creates multiple actions based on the files within one or more input directories, to output one or more output directories.∑0
runë/
©
outputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s'List of the output files of the action.(
Ù
inputs≥<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <a class="anchor" href="../builtins/depset.html">depset</a>0List or depset of the input files of the action."[]
ó
unused_inputs_listM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>´File containing list of inputs unused by the action. <p>The content of this file (generally one of the outputs of the action) corresponds to  the list of input files that were not used during the whole action execution. Any change in those files must not affect in any way the outputs of the action."None
é

executableÃ<a class="anchor" href="../builtins/File.html">File</a>; or <a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../providers/FilesToRunProvider.html">FilesToRunProvider</a>/The executable file to be called by the action.(
¡
toolsw<a class="anchor" href="../core/list.html">sequence</a>; or <a class="anchor" href="../builtins/depset.html">depset</a>µList or <a href="../builtins/depset.html"><code>depset</code></a> of any tools needed by the action. Tools are executable inputs that may have their own runfiles which are automatically made available to the action. <p>
When a list is provided, it can be a heterogenous collection of:
<ul>
    <li><code>File</code>s</li>
    <li><code>FilesToRunProvider</code> instances</li>
    <li><code>depset</code>s of <code>File</code>s</li>
</ul>
<code>File</code>s from <a href="../builtins/ctx#executable"><code>ctx.executable</code></a> and <code>FilesToRunProvider</code>s which are directly in the list will have their runfiles automatically added. All tools are implicitly added as inputs.
</p>
"unbound
≈
	arguments7<a class="anchor" href="../core/list.html">sequence</a>{Command line arguments of the action. Must be a list of strings or <a href="#args"><code>actions.args()</code></a> objects."[]
©
mnemonicM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>HA one-word description of the action, for example, CppCompile or GoLink."None
Í
progress_messageM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>ÄProgress message to show to the user during the build, for example, "Compiling foo.cc to create foo.o". The message may contain <code>%{label}</code>, <code>%{input}</code>, or <code>%{output}</code> patterns, which are substituted with label string, first input, or output's path, respectively. Prefer to use patterns instead of static strings, because the former are more efficient."None
Á
use_default_shell_env3<a class="anchor" href="../core/bool.html">bool</a>ëWhether the action should use the default shell environment, which consists of a few OS-dependent variables as well as variables set via <a href="/reference/command-line-reference#flag--action_env"><code>--action_env</code></a>.<p>If both <code>use_default_shell_env</code> and <code>env</code> are set to <code>True</code>, values set in <code>env</code> will overwrite the default shell environment."False
≥
envI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>⁄Sets the dictionary of environment variables.<p>If both <code>use_default_shell_env</code> and <code>env</code> are set to <code>True</code>, values set in <code>env</code> will overwrite the default shell environment."None
∆
execution_requirementsI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>[Information for scheduling the action. See <a href="#common.tags">tags</a> for useful keys."None
Å
input_manifestsM<a class="anchor" href="../core/list.html">sequence</a>; or <code>None</code>Legacy argument. Ignored."None
◊

exec_groupM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>tRuns the action on the given exec group's execution platform. If none, uses the target's default execution platform."None
È
shadowed_action;<a class="anchor" href="../builtins/Action.html">Action</a>íRuns the action using the given shadowed action's inputs and environment added to the action's inputs list and environment. The action environment can overwrite any of the shadowed action's environment variables. If none, uses only the action's inputs and given environment."None
„
resource_setcallable; or <code>None</code>¨A callback function that returns a resource set dictionary, used to estimate resource usage at execution time if this action is run locally.<p>The function accepts two positional arguments: a string representing an OS name (e.g. "osx"), and an integer representing the number of inputs to the action. The returned dictionary may contain the following entries, each of which may be a float or an int:<ul><li>"cpu": number of CPUs; default 1<li>"memory": in MB; default 250<li>"local_test": number of local tests; default 1</ul><p>If this parameter is set to <code>None</code> , the default values are used.<p>The callback must be top-level (lambda and nested functions aren't allowed)."None
¯
	toolchainã<a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>”<p>Toolchain type of the executable or tools used in this action.</p><p>If executable and tools are not coming from a toolchain, set this parameter to `None`.</p><p>If executable and tools are coming from a toolchain, toolchain type must be set so that the action executes on the correct execution platform.</p><p>Note that the rule which creates this action needs to define this toolchain inside its 'rule()' function.</p><p>When `toolchain` and `exec_group` parameters are both set, `exec_group` will be used. An error is raised in case the `exec_group` doesn't specify the same toolchain.</p>"unboundNoneType"õCreates an action that runs an executable. <a href="https://github.com/bazelbuild/examples/tree/main/rules/actions_run/execute.bzl">See example of use</a>.é7
	run_shell‡5
©
outputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s'List of the output files of the action.(
Ù
inputs≥<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <a class="anchor" href="../builtins/depset.html">depset</a>0List or depset of the input files of the action."[]
˛
tools≥<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <a class="anchor" href="../builtins/depset.html">depset</a>µList or <a href="../builtins/depset.html"><code>depset</code></a> of any tools needed by the action. Tools are executable inputs that may have their own runfiles which are automatically made available to the action. <p>
When a list is provided, it can be a heterogenous collection of:
<ul>
    <li><code>File</code>s</li>
    <li><code>FilesToRunProvider</code> instances</li>
    <li><code>depset</code>s of <code>File</code>s</li>
</ul>
<code>File</code>s from <a href="../builtins/ctx#executable"><code>ctx.executable</code></a> and <code>FilesToRunProvider</code>s which are directly in the list will have their runfiles automatically added. All tools are implicitly added as inputs.
</p>
"unbound
â
	arguments7<a class="anchor" href="../core/list.html">sequence</a>æCommand line arguments of the action. Must be a list of strings or <a href="#args"><code>actions.args()</code></a> objects.<p>Bazel passes the elements in this attribute as arguments to the command.The command can access these arguments using shell variable substitutions such as <code>$1</code>, <code>$2</code>, etc. Note that since Args objects are flattened before indexing, if there is an Args object of unknown size then all subsequent strings will be at unpredictable indices. It may be useful to use <code>$@</code> (to retrieve all arguments) in conjunction with Args objects of indeterminate size.<p>In the case where <code>command</code> is a list of strings, this parameter may not be used."[]
©
mnemonicM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>HA one-word description of the action, for example, CppCompile or GoLink."None
¢
commandØ<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s‚
Shell command to execute. This may either be a string (preferred) or a sequence of strings <b>(deprecated)</b>.<p>If <code>command</code> is a string, then it is executed as if by <code>sh -c &lt;command&gt; "" &lt;arguments&gt;</code> -- that is, the elements in <code>arguments</code> are made available to the command as <code>$1</code>, <code>$2</code> (or <code>%1</code>, <code>%2</code> if using Windows batch), etc. If <code>arguments</code> contains any <a href="#args"><code>actions.args()</code></a> objects, their contents are appended one by one to the command line, so <code>$</code><i>i</i> can refer to individual strings within an Args object. Note that if an Args object of unknown size is passed as part of <code>arguments</code>, then the strings will be at unknown indices; in this case the <code>$@</code> shell substitution (retrieve all arguments) may be useful.<p><b>(Deprecated)</b> If <code>command</code> is a sequence of strings, the first item is the executable to run and the remaining items are its arguments. If this form is used, the <code>arguments</code> parameter must not be supplied. <i>Note that this form is deprecated and will soon be removed. It is disabled with `--incompatible_run_shell_command_string`. Use this flag to verify your code is compatible. </i><p>Bazel uses the same shell to execute the command as it does for genrules.(
Í
progress_messageM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>ÄProgress message to show to the user during the build, for example, "Compiling foo.cc to create foo.o". The message may contain <code>%{label}</code>, <code>%{input}</code>, or <code>%{output}</code> patterns, which are substituted with label string, first input, or output's path, respectively. Prefer to use patterns instead of static strings, because the former are more efficient."None
Á
use_default_shell_env3<a class="anchor" href="../core/bool.html">bool</a>ëWhether the action should use the default shell environment, which consists of a few OS-dependent variables as well as variables set via <a href="/reference/command-line-reference#flag--action_env"><code>--action_env</code></a>.<p>If both <code>use_default_shell_env</code> and <code>env</code> are set to <code>True</code>, values set in <code>env</code> will overwrite the default shell environment."False
≥
envI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>⁄Sets the dictionary of environment variables.<p>If both <code>use_default_shell_env</code> and <code>env</code> are set to <code>True</code>, values set in <code>env</code> will overwrite the default shell environment."None
∆
execution_requirementsI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>[Information for scheduling the action. See <a href="#common.tags">tags</a> for useful keys."None
Å
input_manifestsM<a class="anchor" href="../core/list.html">sequence</a>; or <code>None</code>Legacy argument. Ignored."None
◊

exec_groupM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>tRuns the action on the given exec group's execution platform. If none, uses the target's default execution platform."None
Â
shadowed_action;<a class="anchor" href="../builtins/Action.html">Action</a>éRuns the action using the given shadowed action's discovered inputs added to the action's inputs list. If none, uses only the action's inputs."None
ª
resource_setcallable; or <code>None</code>ÑA callback function for estimating resource usage if run locally. See<a href="#run.resource_set"><code>ctx.actions.run()</code></a>."None
¯
	toolchainã<a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>”<p>Toolchain type of the executable or tools used in this action.</p><p>If executable and tools are not coming from a toolchain, set this parameter to `None`.</p><p>If executable and tools are coming from a toolchain, toolchain type must be set so that the action executes on the correct execution platform.</p><p>Note that the rule which creates this action needs to define this toolchain inside its 'rule()' function.</p><p>When `toolchain` and `exec_group` parameters are both set, `exec_group` will be used. An error is raised in case the `exec_group` doesn't specify the same toolchain.</p>"unboundNoneType"ùCreates an action that runs a shell command. <a href="https://github.com/bazelbuild/examples/tree/main/rules/shell_command/rules.bzl">See example of use</a>.Ö
symlink·
_
output7<a class="anchor" href="../builtins/File.html">File</a>The output of this action.(
ì
target_fileM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>/The File that the output symlink will point to."None
 
target_pathM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>fThe exact path that the output symlink will point to. No normalization or other processing is applied."None
∫
target_typeM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>’May only be used with <code>target_path</code>, not <code>target_file</code>. If specified, it must be one of 'file' or 'directory', indicating the target path's expected type.<p>On Windows, this determines which kind of filesystem object to create (junction for a directory, symlink for a file). It has no effect on other operating systems."None
±
is_executable3<a class="anchor" href="../core/bool.html">bool</a>„May only be used with <code>target_file</code>, not <code>target_path</code>. If true, when the action is executed, the <code>target_file</code>'s path is checked to confirm that it is executable, and an error is reported if it is not. Setting <code>is_executable</code> to False does not mean the target is not executable, just that no verification is done.<p>This feature does not make sense for <code>target_path</code> because dangling symlinks might not exist at build time.</p>"False
ü
progress_messageM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>6Progress message to show to the user during the build."NoneNoneType"ïCreates an action that writes a symlink in the file system.<p>This function must be called with exactly one of <code>target_file</code> or <code>target_path</code> specified.</p><p>When you use <code>target_file</code>, declare <code>output</code> with <a href="#declare_file"><code>declare_file()</code></a> or <a href="#declare_directory"><code>declare_directory()</code></a> and match the type of <code>target_file</code>. This makes the symlink point to <code>target_file</code>. Bazel invalidates the output of this action whenever the target of the symlink or its contents change.</p><p>Otherwise, when you use <code>target_path</code>, declare <code>output</code> with <a href="#declare_symlink"><code>declare_symlink()</code></a>). In this case, the symlink points to <code>target_path</code>. Bazel never resolves the symlink and the output of this action is invalidated only when the text contents of the symlink (that is, the value of <code>readlink()</code>) changes. In particular, this can be used to create a dangling symlink.</p>g
template_dictTemplateDict"FReturns a TemplateDict object for memory-efficient template expansion.ƒ
write˛
U
output7<a class="anchor" href="../builtins/File.html">File</a>The output file.(
Ú
contents<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Args.html">Args</a>pthe contents of the file. May be a either a string or an <a href="#args"><code>actions.args()</code></a> object.(
z
is_executable3<a class="anchor" href="../core/bool.html">bool</a>-Whether the output file should be executable."False
©
mnemonicM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>HA one-word description of the action, for example, CppCompile or GoLink."NoneNoneType"πCreates a file write action. When the action is executed, it will write the given content to a file. This is used to generate files using information available in the analysis phase. If the file is large and with a lot of static content, consider using <a href="#expand_template"><code>expand_template</code></a>.ãModule providing functions to create actions. Access this module using <a href="../builtins/ctx.html#actions"><code>ctx.actions</code></a>.
Ê
apple_platformá
	is_devicebool"tReturns <code>True</code> if this platform is a device platform or <code>False</code> if it is a simulator platform.B
namestring"2Returns the name aka starlarkKey of this platform.Ò
name_in_pliststring"◊The name of the platform as it appears in the <code>CFBundleSupportedPlatforms</code> entry of an Info.plist file and in Xcode's platforms directory, without the extension (for example, <code>iPhoneOS</code> or <code>iPhoneSimulator</code>).<br>This name, when converted to lowercase (e.g., <code>iphoneos</code>, <code>iphonesimulator</code>), can be passed to Xcode's command-line tools like <code>ibtool</code> and <code>actool</code> when they expect a platform name.D
platform_typestring"+Returns the platform type of this platform.À	Corresponds to Xcode's notion of a platform as would be found in <code>Xcode.app/Contents/Developer/Platforms</code>. Each platform represents an Apple platform type (such as iOS or tvOS) combined with one or more related CPU architectures. For example, the iOS simulator platform supports <code>x86_64</code> and <code>i386</code> architectures.<p>Specific instances of this type can be retrieved from the fields of the <a href='../toplevel/apple_common.html#platform'>apple_common.platform</a> struct:<br><ul><li><code>apple_common.platform.ios_device</code></li><li><code>apple_common.platform.ios_simulator</code></li><li><code>apple_common.platform.macos</code></li><li><code>apple_common.platform.tvos_device</code></li><li><code>apple_common.platform.tvos_simulator</code></li><li><code>apple_common.platform.watchos_device</code></li><li><code>apple_common.platform.watchos_simulator</code></li></ul><p>More commonly, however, the <a href='../fragments/apple.html'>apple</a> configuration fragment has fields/methods that allow rules to determine the platform for which a target is being built.<p>Example:<br><pre class='language-python'>
p = apple_common.platform.ios_device
print(p.name_in_plist)  # 'iPhoneOS'
</pre>
ÛÖ
Args¿
addç
Ñ
arg_name_or_valueÏIf two positional parameters are passed this is interpreted as the arg name. The arg name is added before the value without any processing. If only one positional parameter is passed, it is interpreted as <code>value</code> (see below).(
√
value∞The object to append. It will be converted to a string using the standard conversion mentioned above. Since there is no <code>map_each</code> parameter for this function, <code>value</code> should be either a string or a <code>File</code>. A list, tuple, depset, or directory <code>File</code> must be passed to <a href='#add_all'><code>add_all()</code> or <a href='#add_joined'><code>add_joined()</code></a> instead of this method."unbound
∑
formatM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>XA format string pattern, to be applied to the stringified version of <code>value</code>."NoneArgs")Appends an argument to this command line.™0
add_all”%
Ó
arg_name_or_values’If two positional parameters are passed this is interpreted as the arg name. The arg name is added before the <code>values</code> as a separate argument without any processing. This arg name will not be added if <code>omit_if_empty</code> is true (the default) and no other items are appended (as happens if <code>values</code> is empty or all of its items are filtered). If only one positional parameter is passed, it is interpreted as <code>values</code> (see below).(
ƒ
valuesw<a class="anchor" href="../core/list.html">sequence</a>; or <a class="anchor" href="../builtins/depset.html">depset</a>8The list, tuple, or depset whose items will be appended."unbound
“
map_eachcallable; or <code>None</code>üA function that converts each item to zero or more strings, which may be further processed before appending. If this param is not provided, the standard conversion is used.<p>The function is passed either one or two positional arguments: the item to convert, followed by an optional <a href='../builtins/DirectoryExpander.html'><code>DirectoryExpander</code></a>. The second argument will be passed only if the supplied function is user-defined (not built-in) and declares more than one parameter.<p>The return value's type depends on how many arguments are to be produced for the item:<ul><li>In the common case when each item turns into one string, the function     should return that string.<li>If the item is to be filtered out entirely, the function should return     <code>None</code>.<li>If the item turns into multiple strings, the function returns a list of     those strings.</ul>Returning a single string or <code>None</code> has the same effect as returning a list of length 1 or length 0 respectively. However, it is more efficient and readable to avoid creating a list where it is not needed.<p>Ordinarily, items that are directories are automatically expanded to their contents when <code>expand_directories=True</code> is set. However, this will not expand directories contained inside other values -- for instance, when the items are structs that have directories as fields. In this situation, the <code>DirectoryExpander</code> argument can be applied to manually obtain the files of a given directory.<p>To avoid unintended retention of large analysis-phase data structures into the execution phase, the <code>map_each</code> function must be declared by a top-level <code>def</code> statement; it may not be a nested function closure by default.<p><i>Warning:</i> <a href='../globals/all.html#print'><code>print()</code></a> statements that are executed during the call to <code>map_each</code> will not produce any visible output."None
à
format_eachM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>£An optional format string pattern, applied to each string returned by the <code>map_each</code> function. The format string must have exactly one '%s' placeholder."None
≈
before_eachM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>aAn optional argument to append before each argument derived from <code>values</code> is appended."None
Ü
omit_if_empty3<a class="anchor" href="../core/bool.html">bool</a>πIf true, if there are no arguments derived from <code>values</code> to be appended, then all further processing is suppressed and the command line will be unchanged. If false, the arg name and <code>terminate_with</code>, if provided, will still be appended regardless of whether or not there are other arguments."True
¸
uniquify3<a class="anchor" href="../core/bool.html">bool</a>≥If true, duplicate arguments that are derived from <code>values</code> will be omitted. Only the first occurrence of each argument will remain. Usually this feature is not needed because depsets already omit duplicates, but it can be useful if <code>map_each</code> emits the same string for multiple items."False
·
expand_directories3<a class="anchor" href="../core/bool.html">bool</a>èIf true, any directories in <code>values</code> will be expanded to a flat list of files. This happens before <code>map_each</code> is applied."True
‰
terminate_withM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>¸An optional argument to append after all other arguments. This argument will not be added if <code>omit_if_empty</code> is true (the default) and no other items are appended (as happens if <code>values</code> is empty or all of its items are filtered)."None
ó
allow_closure3<a class="anchor" href="../core/bool.html">bool</a>…If true, allows the use of closures in function parameters like <code>map_each</code>. Usually this isn't necessary and it risks retaining large analysis-phase data structures into the execution phase."FalseArgs"»
Appends multiple arguments to this command line. The items are processed lazily during the execution phase.<p>Most of the processing occurs over a list of arguments to be appended, as per the following steps:<ol><li>Each directory <code>File</code> item is replaced by all <code>File</code>s recursively contained in that directory.</li><li>If <code>map_each</code> is given, it is applied to each item, and the     resulting lists of strings are concatenated to form the initial argument     list. Otherwise, the initial argument list is the result of applying the     standard conversion to each item.<li>Each argument in the list is formatted with <code>format_each</code>, if     present.<li>If <code>uniquify</code> is true, duplicate arguments are removed. The first     occurrence is the one that remains.<li>If a <code>before_each</code> string is given, it is inserted as a new     argument before each existing argument in the list. This effectively doubles     the number of arguments to be appended by this point.<li>Except in the case that the list is empty and <code>omit_if_empty</code> is     true (the default), the arg name and <code>terminate_with</code> are     inserted as the first and last arguments, respectively, if they are given.</ol>Note that empty strings are valid arguments that are subject to all these processing steps.¥

add_joinedà
ˇ
arg_name_or_valuesÊIf two positional parameters are passed this is interpreted as the arg name. The arg name is added before <code>values</code> without any processing. This arg will not be added if <code>omit_if_empty</code> is true (the default) and there are no strings derived from <code>values</code> to join together (which can happen if <code>values</code> is empty or all of its items are filtered). If only one positional parameter is passed, it is interpreted as <code>values</code> (see below).(
¬
valuesw<a class="anchor" href="../core/list.html">sequence</a>; or <a class="anchor" href="../builtins/depset.html">depset</a>6The list, tuple, or depset whose items will be joined."unbound
°
	join_with7<a class="anchor" href="../core/string.html">string</a>ÿA delimiter string used to join together the strings obtained from applying <code>map_each</code> and <code>format_each</code>, in the same manner as <a href='../core/string.html#join'><code>string.join()</code></a>.(
s
map_eachcallable; or <code>None</code>ASame as for <a href='#add_all.map_each'><code>add_all</code></a>."None
®
format_eachM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>DSame as for <a href='#add_all.format_each'><code>add_all</code></a>."None
ﬂ
format_joinedM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>yAn optional format string pattern applied to the joined string. The format string must have exactly one '%s' placeholder."None
—
omit_if_empty3<a class="anchor" href="../core/bool.html">bool</a>ÑIf true, if there are no strings to join together (either because <code>values</code> is empty or all its items are filtered), then all further processing is suppressed and the command line will be unchanged. If false, then even if there are no strings to join together, two arguments will be appended: the arg name followed by an empty string (which is the logical join of zero strings)."True
â
uniquify3<a class="anchor" href="../core/bool.html">bool</a>ASame as for <a href='#add_all.uniquify'><code>add_all</code></a>."False
ú
expand_directories3<a class="anchor" href="../core/bool.html">bool</a>KSame as for <a href='#add_all.expand_directories'><code>add_all</code></a>."True
ì
allow_closure3<a class="anchor" href="../core/bool.html">bool</a>FSame as for <a href='#add_all.allow_closure'><code>add_all</code></a>."FalseArgs"öAppends an argument to this command line by concatenating together multiple values using a separator. The items are processed lazily during the execution phase.<p>Processing is similar to <a href='#add_all'><code>add_all()</code></a>, but the list of arguments derived from <code>values</code> is combined into a single argument as if by <code>join_with.join(...)</code>, and then formatted using the given <code>format_joined</code> string template. Unlike <code>add_all()</code>, there is no <code>before_each</code> or <code>terminate_with</code> parameter since these are not generally useful when the items are combined into a single argument.<p>If after filtering there are no strings to join into an argument, and if <code>omit_if_empty</code> is true (the default), no processing is done. Otherwise if there are no strings to join but <code>omit_if_empty</code> is false, the joined string will be an empty string.π
set_param_file_formatÏ
„
format7<a class="anchor" href="../core/string.html">string</a>ùMust be one of:<ul><li>"multiline": Each item (argument name or value) is written verbatim to the param file with a newline character following it.</li><li>"shell": Same as "multiline", but the items are shell-quoted</li><li>"flag_per_line": Same as "multiline", but (1) only flags (beginning with '--') are written to the param file, and (2) the values of the flags, if any, are written on the same line with a '=' separator. This is the format expected by the Abseil flags library.</li></ul><p>The format defaults to "shell" if not called.(Args"1Sets the format of the param file, if one is usedπ
use_param_file°
Æ
param_file_arg7<a class="anchor" href="../core/string.html">string</a>‡A format string with a single "%s". If the args are spilled to a params file then they are replaced with an argument consisting of this string formatted with the path of the params file.<p>For example, if the args are spilled to a params file "params.txt", then specifying "--file=%s" would cause the action command line to contain "--file=params.txt".(
Á

use_always3<a class="anchor" href="../core/bool.html">bool</a>úWhether to always spill the args to a params file. If false, bazel will decide whether the arguments need to be spilled based on your system and arg length."FalseArgs"ÇSpills the args to a params file, replacing them with a pointer to the param file. Use when your args may be too large for the system's command length limits.<p>Bazel may choose to elide writing the params file to the output tree during execution for efficiency. If you are debugging actions and want to inspect the param file, pass <code>--materialize_param_files</code> to your build.À%An object that encapsulates, in a memory-efficient way, the data needed to build part or all of a command line.<p>It often happens that an action requires a large command line containing values accumulated from transitive dependencies. For example, a linker command line might list every object file needed by all of the libraries being linked. It is best practice to store such transitive data in <a href='../builtins/depset.html'><code>depset</code></a>s, so that they can be shared by multiple targets. However, if the rule author had to convert these depsets into lists of strings in order to construct an action command line, it would defeat this memory-sharing optimization.<p>For this reason, the action-constructing functions accept <code>Args</code> objects in addition to strings. Each <code>Args</code> object represents a concatenation of strings and depsets, with optional transformations for manipulating the data. <code>Args</code> objects do not process the depsets they encapsulate until the execution phase, when it comes time to calculate the command line. This helps defer any expensive copying until after the analysis phase is complete. See the <a href='https://bazel.build/rules/performance'>Optimizing Performance</a> page for more information.<p><code>Args</code> are constructed by calling <a href='../builtins/actions.html#args'><code>ctx.actions.args()</code></a>. They can be passed as the <code>arguments</code> parameter of <a href='../builtins/actions.html#run'><code>ctx.actions.run()</code></a> or <a href='../builtins/actions.html#run_shell'><code>ctx.actions.run_shell()</code></a>. Each mutation of an <code>Args</code> object appends values to the eventual command line.<p>The <code>map_each</code> feature allows you to customize how items are transformed into strings. If you do not provide a <code>map_each</code> function, the standard conversion is as follows: <ul><li>Values that are already strings are left as-is.<li><a href='../builtins/File.html'><code>File</code></a> objects are turned into their     <code>File.path</code> values.<li><a href='../builtins/Label.html'><code>Label</code></a> objects are turned into a string representation that resolves back to the same object when resolved in the context of the main repository. If possible, the string representation uses the apparent name of a repository in favor of the repository's canonical name, which makes this representation suited for use in BUILD files. While the exact form of the representation is not guaranteed, typical examples are <code>//foo:bar</code>, <code>@repo//foo:bar</code> and <code>@@canonical_name+//foo:bar.bzl</code>.<li>All other types are turned into strings in an <i>unspecified</i> manner. For     this reason, you should avoid passing values that are not of string or     <code>File</code> type to <code>add()</code>, and if you pass them to     <code>add_all()</code> or <code>add_joined()</code> then you should provide a     <code>map_each</code> function.</ul><p>When using string formatting (<code>format</code>, <code>format_each</code>, and <code>format_joined</code> params of the <code>add*()</code> methods), the format template is interpreted in the same way as <code>%</code>-substitution on strings, except that the template must have exactly one substitution placeholder and it must be <code>%s</code>. Literal percents may be escaped as <code>%%</code>. Formatting is applied after the value is converted to a string as per the above.<p>Each of the <code>add*()</code> methods have an alternate form that accepts an extra positional parameter, an "arg name" string to insert before the rest of the arguments. For <code>add_all</code> and <code>add_joined</code> the extra string will not be added if the sequence turns out to be empty. For instance, the same usage can add either <code>--foo val1 val2 val3 --bar</code> or just <code>--bar</code> to the command line, depending on whether the given sequence contains <code>val1..val3</code> or is empty.<p>If the size of the command line can grow longer than the maximum size allowed by the system, the arguments can be spilled over into parameter files. See <a href='#use_param_file'><code>use_param_file()</code></a> and <a href='#set_param_file_format'><code>set_param_file_format()</code></a>.<p>Example: Suppose we wanted to generate the command line: <pre>
--foo foo1.txt foo2.txt ... fooN.txt --bar bar1.txt,bar2.txt,...,barM.txt --baz
</pre>We could use the following <code>Args</code> object: <pre class=language-python>
# foo_deps and bar_deps are depsets containing
# File objects for the foo and bar .txt files.
args = ctx.actions.args()
args.add_all("--foo", foo_deps)
args.add_joined("--bar", bar_deps, join_with=",")
args.add("--baz")
ctx.actions.run(
  ...
  arguments = [args],
  ...
)
</pre>
‰
AspectŸFor more information about Aspects, please consult the
<a href="../globals/bzl.html#aspect">documentation of the aspect function</a> or the <a href="https://bazel.build/extending/aspects">introduction to Aspects</a>.

ç
	AttributeˇRepresentation of a definition of an attribute. Use the <a href="../toplevel/attr.html">attr</a> module to create an Attribute. They are only for use with a <a href="../globals/bzl.html#rule">rule</a> or an <a href="../globals/bzl.html#aspect">aspect</a>.
À
bazel_module8
is_rootbool"'Whether this module is the root module.'
namestring"The name of the module.l
tagsbazel_module_tags"QThe tags in the module related to the module extension currently being processed.-
versionstring"The version of the module.;Represents a Bazel module in the external dependency graph.
Ã
bazel_module_tags∂Contains the tags in a module for the module extension currently being processed. This object has a field for each tag class of the extension, and the value of the field is a list containing an object for each tag instance. This "tag instance" object in turn has a field for each attribute of the tag class.

When passed as positional arguments to <code>print()</code> or <code>fail()</code>, tag instance objects turn into a meaningful string representation of the form "'install' tag at /home/user/workspace/MODULE.bazel:3:4". This can be used to construct error messages that point to the location of the tag in the module file, e.g. <code>fail("Conflict between", tag1, "and", tag2)</code>.
—
BuildSetting¿The descriptor for a single piece of configuration information. If configuration is a key-value map of settings like {'cpu': 'ppc', 'copt': '-DFoo'}, this describes a single entry in that map.
ü
CcCompilationOutputs*
objectssequence"Non-PIC object files.*
pic_objectssequence"PIC object files./Helper class containing CC compilation outputs.
Ë
CcLinkingOutputs5

executableFile"!Represents the linked executable.l
library_to_linkLibraryToLink"J<code>LibraryToLink</code> for including these outputs in further linking./Helper class containing CC compilation outputs.
ë
CompilationContextÿ
definesdepset"ƒReturns the set of defines needed to compile this target. Each define is a string. These values are propagated to the target's transitive dependents, that is, any rules that depend on this target.◊
direct_headerslist"æReturns the list of modular headers that are declared by this target. This includes both public headers (such as those listed in "hdrs") and private headers (such as those listed in "srcs").Ü
direct_private_headerslist"fReturns the list of modular private headers (those listed in "srcs") that are declared by this target.Ñ
direct_public_headerslist"eReturns the list of modular public headers (those listed in "hdrs") that are declared by this target.e
direct_textual_headerslist"EReturns the list of textual headers that are declared by this target.ü
external_includesdepset"ÅReturns the set of search paths (as strings) for external header files referenced by angle bracket. Usually passed with -isystem.~
framework_includesdepset"`Returns the set of search paths (as strings) for framework header files. Usually passed with -F.L
headersdepset"9Returns the set of headers needed to compile this target.ó
includesdepset"ÇReturns the set of search paths (as strings) for header files referenced both by angle bracket and quotes. Usually passed with -I.≥
local_definesdepset"ôReturns the set of defines needed to compile this target. Each define is a string. These values are not propagated to the target's transitive dependents.Á
quote_includesdepset"ÃReturns the set of search paths (as strings) for header files referenced by quotes, e.g. #include "foo/bar/header.h". They can be either relative to the exec root or absolute. Usually passed with -iquote.˜
system_includesdepset"€Returns the set of search paths (as strings) for header files referenced by angle brackets, e.g. #include &lt;foo/bar/header.h&gt;. They can be either relative to the exec root or absolute. Usually passed with -isystem.H
validation_artifactsdepset"(Returns the set of validation artifacts.aImmutable store of information needed for C++ compilation that is aggregated across dependencies.
ﬂ
configurationÃ
coverage_enabledbool"±A boolean that tells whether code coverage is enabled for this run. Note that this does not compute whether a specific rule should be instrumented for code coverage data collection. For that, see the <a href="../builtins/ctx.html#coverage_instrumented"><code>ctx.coverage_instrumented</code></a> function.Ö
default_shell_envdict"jA dictionary representing the static local shell environment. It maps variables to their values (strings).i
host_path_separatorstring"JReturns the separator for PATH environment variable, which is ':' on Unix.Â
short_idstring"–A short identifier for this configuration understood by the <code>config</code> and </code>query</code> subcommands. 
<p>Use this to distinguish different configurations for the same target in a way that is friendly to humans and tool usage, for example in an aspect used by an IDE. Keep in mind the following caveats: <ul>   <li>The value may differ across Bazel versions, including patch releases.   <li>The value encodes the value of <b>every</b> flag, including those that aren't       otherwise relevant for the current target and may thus invalidate caches more       frequently. </ul>
«
test_envdict"¥A dictionary containing user-specified test environment variables and their values, as set by the <code>--test_env</code> options. DO NOT USE! This is not the complete environment!ŸThis object holds information about the environment in which the build is running. See the <a href='https://bazel.build/extending/rules#configurations'>Rules page</a> for more on the general concept of configurations.
»ã
ctxb
actionsactions"NContains methods for declaring output files and the actions that produce them.{

aspect_idslist"gA list of ids for all aspects applied to the target. Only available in aspect implementation functions.±
attrstruct"†A struct to access the values of the <a href='https://bazel.build/extending/rules#attributes'>attributes</a>. The values are provided by the user (if not, a default value is used). The attributes of the struct and the types of their values correspond to the keys and values of the <a href='../globals/bzl.html#rule.attrs'><code>attrs</code> dict</a> provided to the <a href='../globals/bzl.html#rule'><code>rule</code> function</a>. <a href="https://github.com/bazelbuild/examples/blob/main/rules/attributes/printer.bzl">See example of use</a>.9
bin_dirroot"(The root corresponding to bin directory.ù
build_file_pathstring"ÅDeprecated: Use <code>ctx.label.package + '/BUILD'</code>. The path to the BUILD file for this rule, relative to the source root.¢
build_setting_valueunknown"ÅValue of the build setting represented by the current target. If this isn't the context for an instance of a rule that sets the <a href="https://bazel.build/extending/config#rule-parameter"><code>build_setting</code></a> attribute, reading this is an error.˚
configurationconfiguration"⁄The current target's build configuration. See the <a href="../builtins/configuration.html">Starlark configuration type</a> and <a href="/extending/rules#configurations">configuration documentation</a> for more details.ü
coverage_instrumented∂
≠
targetQ<a class="anchor" href="../builtins/Target.html">Target</a>; or <code>None</code>JA Target specifying a rule. If not provided, defaults to the current rule."Nonebool"ÃReturns whether code coverage instrumentation should be generated when performing compilation actions for this rule or, if <code>target</code> is provided, the rule specified by that Target. (If a non-rule or a Starlark rule Target is provided, this returns False.) Checks if the sources of the current rule (if no Target is provided) or the sources of Target should be instrumented based on the --instrumentation_filter and --instrument_test_targets config settings. This differs from <code>coverage_enabled</code> in the <a href="../builtins/configuration.html">configuration</a>, which notes whether coverage data collection is enabled for the entire run, but not whether a specific target should be instrumented.Ë
created_actionsStarlarkValue"√For rules with <a href="../globals/bzl.html#rule._skylark_testable">_skylark_testable</a> set to <code>True</code>, this returns an <code>Actions</code> provider representing all actions created so far for the current rule. For all other rules, returns <code>None</code>. Note that the provider is not updated when subsequent actions are created, so you will have to call this function again if you wish to inspect them. <br/><br/>This is intended to help write tests for rule-implementation helper functions, which may take in a <code>ctx</code> object and create actions on it.f
disabled_featureslist"KThe set of features that are explicitly disabled by the user for this rule.≤
exec_groupsExecGroupCollection"çA collection of the execution groups available for this rule, indexed by their name. Access with <code>ctx.exec_groups[name_of_group]</code>.‚

executablestruct"ÀA <code>struct</code> containing executable files defined in <a href='../toplevel/attr.html#label'>label type attributes</a> marked as <a href='../toplevel/attr.html#label.executable'><code>executable=True</code></a>. The struct fields correspond to the attribute names. Each value in the struct is either a <a href='../builtins/File.html'><code>File</code></a> or <code>None</code>. If an optional attribute is not specified in the rule then the corresponding struct value is <code>None</code>. If a label type is not marked as <code>executable=True</code>, no corresponding struct field is generated. <a href="https://github.com/bazelbuild/examples/blob/main/rules/actions_run/execute.bzl">See example of use</a>.Å
expand_locationË
Z
input7<a class="anchor" href="../core/string.html">string</a>String to be expanded.(
Å
targetsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Target.html">Target</a>s¯List of targets for additional lookup information. These are expanded as follows: A target with a single file in <code>DefaultInfo.files</code> expands to that file. Other targets expand to their <code>DefaultInfo.executable</code> file if set and if <code>--incompatible_locations_prefers_executable</code> is enabled, otherwise they expand to <code>DefaultInfo.files</code>."[]string"ÇExpands all <code>$(location ...)</code> templates in the given string by replacing <code>$(location //x)</code> with the path of the output file of target //x. Expansion only works for labels that point to direct dependencies of this rule or that are explicitly listed in the optional argument <code>targets</code>. <br/><br/><code>$(location ...)</code> will cause an error if the referenced target has multiple outputs. In this case, please use <code>$(locations ...)</code> since it produces a space-separated list of output paths. It can be safely used for a single output file, too.<br/><br/>This function is useful to let the user specify a command in a BUILD file (like for <code>genrule</code>). In other cases, it is often better to manipulate labels directly.•
expand_make_variablesÆ
z
attribute_name7<a class="anchor" href="../core/string.html">string</a>-The attribute name. Used for error reporting.(
é
command7<a class="anchor" href="../core/string.html">string</a>HThe expression to expand. It can contain references to "Make variables".(
ñ
additional_substitutions3<a class="anchor" href="../core/dict.html">dict</a>CAdditional substitutions to make beyond the default make variables.(string"⁄<b>Deprecated.</b> Use <a href="../builtins/ctx.html#var">ctx.var</a> to access the variables instead.<br>Returns a string after expanding all references to "Make variables". The variables must have the following format: <code>$(VAR_NAME)</code>. Also, <code>$$VAR_NAME</code> expands to <code>$VAR_NAME</code>. Examples:<pre class=language-python>
ctx.expand_make_variables("cmd", "$(MY_VAR)", {"MY_VAR": "Hi"})  # == "Hi"
ctx.expand_make_variables("cmd", "$$PWD", {})  # == "$PWD"
</pre>Additional variables may come from other places, such as configurations. Note that this function is experimental.»
featureslist"µThe set of features that are explicitly enabled by the user for this rule. <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/rule.bzl">See example of use</a>.Ÿ
filestruct"»A <code>struct</code> containing files defined in <a href='../toplevel/attr.html#label'>label type attributes</a> marked as <a href='../toplevel/attr.html#label.allow_single_file'><code>allow_single_file</code></a>. The struct fields correspond to the attribute names. The struct value is always a <a href='../builtins/File.html'><code>File</code></a> or <code>None</code>. If an optional attribute is not specified in the rule then the corresponding struct value is <code>None</code>. If a label type is not marked as <code>allow_single_file</code>, no corresponding struct field is generated. It is a shortcut for:<pre class=language-python>list(ctx.attr.&lt;ATTR&gt;.files)[0]</pre>In other words, use <code>file</code> to access the (singular) <a href="https://bazel.build/extending/rules#requesting_output_files">default output</a> of a dependency. <a href="https://github.com/bazelbuild/examples/blob/main/rules/expand_template/hello.bzl">See example of use</a>.Œ
filesstruct"ºA <code>struct</code> containing files defined in <a href='../toplevel/attr.html#label'>label</a> or <a href='../toplevel/attr.html#label_list'>label list</a> type attributes. The struct fields correspond to the attribute names. The struct values are <code>list</code> of <a href='../builtins/File.html'><code>File</code></a>s.  It is a shortcut for:<pre class=language-python>[f for t in ctx.attr.&lt;ATTR&gt; for f in t.files]</pre> In other words, use <code>files</code> to access the <a href="https://bazel.build/extending/rules#requesting_output_files"> default outputs</a> of a dependency. <a href="https://github.com/bazelbuild/examples/blob/main/rules/depsets/foo.bzl">See example of use</a>.Y
	fragments	fragments"AAllows access to configuration fragments in target configuration.C
genfiles_dirroot"-The root corresponding to genfiles directory.π
	info_fileFile"•The file that is used to hold the non-volatile workspace status for the current build request. See documentation for --workspace_status_command for more information.A
labelLabel"1The label of the target currently being analyzed.Ö
outputs	structure"ÓA pseudo-struct containing all the predeclared output files, represented by <a href='../builtins/File.html'><code>File</code></a> objects. See the <a href='https://bazel.build/extending/rules#files'>Rules page</a> for more information and examples.<p>This field does not exist on aspect contexts, since aspects do not have predeclared outputs.<p>The fields of this object are defined as follows. It is an error if two outputs produce the same field name or have the same label.<ul><li> If the rule declares an <a href='../globals/bzl.html#rule.outputs'><code>outputs</code></a> dict, then for every entry in the dict, there is a field whose name is the key and whose value is the corresponding <code>File</code>.<li>For every attribute of type <a href='../toplevel/attr.html#output'><code>attr.output</code></a> that the rule declares, there is a field whose name is the attribute's name. If the target specified a label for that attribute, then the field value is the corresponding <code>File</code>; otherwise the field value is <code>None</code>.<li>For every attribute of type <a href='../toplevel/attr.html#output_list'><code>attr.output_list</code></a> that the rule declares, there is a field whose name is the attribute's name. The field value is a list of <code>File</code> objects corresponding to the labels given for that attribute in the target, or an empty list if the attribute was not specified in the target.<li><b>(Deprecated)</b> If the rule is marked <a href='../globals/bzl.html#rule.executable'><code>executable</code></a> or <a href='../globals/bzl.html#rule.test'><code>test</code></a>, there is a field named <code>"executable"</code>, which is the default executable. It is recommended that instead of using this, you pass another file (either predeclared or not) to the <code>executable</code> arg of <a href='../providers/DefaultInfo.html'><code>DefaultInfo</code></a>.</ul>Ä
resolve_command˚
[
command7<a class="anchor" href="../core/string.html">string</a>Command to resolve."''
®
	attributeM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>FName of the associated attribute for which to issue an error, or None."None
¡
expand_locations3<a class="anchor" href="../core/bool.html">bool</a>qShall we expand $(location) variables? See <a href="#expand_location">ctx.expand_location()</a> for more details."False
Ö
make_variablesI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>"Make variables to expand, or None."None
¶
toolsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Target.html">Target</a>s List of tools (list of targets)."[]
´

label_dict3<a class="anchor" href="../core/dict.html">dict</a>dDictionary of resolved labels and the corresponding list of Files (a dict of Label : list of Files)."{}
∆
execution_requirements3<a class="anchor" href="../core/dict.html">dict</a>sInformation for scheduling the action to resolve this command. See <a href="#common.tags">tags</a> for useful keys."{}tuple"Ó<i>(Experimental)</i> Returns a tuple <code>(inputs, command, empty list)</code> of the list of resolved inputs and the argv list for the resolved command both of them suitable for passing as the same-named arguments of the  <code>ctx.action</code> method.<br/><b>Note for Windows users</b>: this method requires Bash (MSYS2). Consider using <code>resolve_tools()</code> instead (if that fits your needs). The empty list is returned as the third member of the tuple for backwards compatibility.Æ
resolve_tools∞
¶
toolsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Target.html">Target</a>s List of tools (list of targets)."[]tuple"ÈReturns a tuple <code>(inputs, empty list)</code> of the depset of resolved inputs required to run the tools, suitable for passing as the same-named argument of the <code>ctx.actions.run</code> and <code>ctx.actions.run_shell</code> methods. <br/><br/>In contrast to <code>ctx.resolve_command</code>, this method does not require that Bash be installed on the machine, so it's suitable for rules built on Windows. The empty list is returned as part of the tuple for backward compatibility.í
rulerule_attributes"yRule attributes descriptor for the rule that the aspect is applied to. Only available in aspect implementation functions.î
runfilesÎ
∞
filess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s.The list of files to be added to the runfiles."[]
»
transitive_filesç<a class="anchor" href="../builtins/depset.html">depset</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <code>None</code>ùThe (transitive) set of files to be added to the runfiles. The depset should use the <code>default</code> order (which, as the name implies, is the default)."None
≠
collect_data3<a class="anchor" href="../core/bool.html">bool</a>‡<b>Use of this parameter is not recommended. See <a href="https://bazel.build/extending/rules#runfiles">runfiles guide</a></b>. <p>Whether to collect the data runfiles from the dependencies in srcs, data and deps attributes."False
≥
collect_default3<a class="anchor" href="../core/bool.html">bool</a>„<b>Use of this parameter is not recommended. See <a href="https://bazel.build/extending/rules#runfiles">runfiles guide</a></b>. <p>Whether to collect the default runfiles from the dependencies in srcs, data and deps attributes."False
„
symlinksø<a class="anchor" href="../core/dict.html">dict</a>; or <a class="anchor" href="../builtins/depset.html">depset</a> of <a class="anchor" href="../builtins/SymlinkEntry.html">SymlinkEntry</a>sêEither a SymlinkEntry depset or the map of symlinks to be added to the runfiles. Symlinks are always added under the main workspace's runfiles directory (e.g. <code>&lt;runfiles_root>/_main/&lt;symlink_path></code>, <b>not</b> the directory corresponding to the current target's repository. See <a href="https://bazel.build/extending/rules#runfiles_symlinks">Runfiles symlinks</a> in the rules guide."{}
î
root_symlinksø<a class="anchor" href="../core/dict.html">dict</a>; or <a class="anchor" href="../builtins/depset.html">depset</a> of <a class="anchor" href="../builtins/SymlinkEntry.html">SymlinkEntry</a>sºEither a SymlinkEntry depset or a map of symlinks to be added to the runfiles. See <a href="https://bazel.build/extending/rules#runfiles_symlinks">Runfiles symlinks</a> in the rules guide."{}runfiles"Creates a runfiles object.≠

split_attrstruct"ñA struct to access the values of attributes with split configurations. If the attribute is a label list, the value of split_attr is a dict of the keys of the split (as strings) to lists of the ConfiguredTargets in that branch of the split. If the attribute is a label, then the value of split_attr is a dict of the keys of the split (as strings) to single ConfiguredTargets. Attributes with split configurations still appear in the attr struct, but their values will be single lists with all the branches of the split merged together.b
super	unknown"NExperimental: Calls parent's implementation function and returns its providersß
target_platform_has_constraint∞
ß
constraintValueV<a class="anchor" href="../providers/ConstraintValueInfo.html">ConstraintValueInfo</a>:The constraint value to check the target platform against.(bool"RReturns true if the given constraint value is part of the current target platform.S

toolchainsToolchainContext"3Toolchains for the default exec group of this rule.F
vardict"9Dictionary (String to String) of configuration variables.∏
version_fileFile"°The file that is used to hold the volatile workspace status for the current build request. See documentation for --workspace_status_command for more information.•
workspace_namestring"äThe name of the workspace, which is effectively the execution root name and runfiles prefix for the main repo. If <code>--enable_bzlmod</code> is on, this is the fixed string <code>_main</code>. Otherwise, this is the workspace name as defined in the WORKSPACE file.µA context object that is passed to the implementation function for a rule or aspect. It provides access to the information and methods needed to analyze the current target.<p>In particular, it lets the implementation function access the current target's label, attributes, configuration, and the providers of its dependencies. It has methods for declaring output files and the actions that produce them.<p>Context objects essentially live for the duration of the call to the implementation function. It is not useful to access these objects outside of their associated function. See the <a href='https://bazel.build/extending/rules#implementation_function'>Rules page</a> for more information.
≥
depsetø
to_listlist"´Returns a list of the elements, without duplicates, in the depset's traversal order. Note that order is unspecified (but deterministic) for elements that were added more than once to the depset. Order is also unspecified for <code>"default"</code>-ordered depsets, and for elements of child depsets whose order differs from that of the parent depset. The list is a copy; modifying it has no effect on the depset and vice versa.Ê<p>A specialized data structure that supports efficient merge operations and has a defined traversal
order. Commonly used for accumulating data from transitive dependencies in rules and aspects. For
more information see <a href="/extending/depsets">here</a>.

<p>The elements of a depset must be hashable and all of the same type (as defined by the built-in
<a href="../globals/all#type"><code>type(x)</code></a> function), but depsets are not simply hash
sets and do not support fast membership tests. If you need a general set datatype, use the core
<a href="../core/set">Starlark set</a> type (available since Bazel 8.1); if your .bzl file needs to
be compatible with older Bazel releases, you can simulate a set by using a dictionary where all keys
map to <code>True</code>.

<p>When tested for truth (that is, when used in a Boolean context such as <code>if d:</code> where
<code>d</code> is a depset), a depset is True if and only if it is non-empty; this check is an O(1)
operation.

<p>Depsets are immutable. They should be created using their
<a href="../globals/bzl.html#depset">constructor function</a> and merged or augmented with other
depsets via the <code>transitive</code> argument.

<p>The <code>order</code> parameter determines the kind of traversal that is done to convert the
depset to an iterable. There are four possible values:

<ul>
  <li>
    <code>"default"</code> (formerly <code>"stable"</code>): Order is unspecified (but
    deterministic).
  </li>
  <li>
    <code>"postorder"</code> (formerly <code>"compile"</code>): A left-to-right post-ordering.
    Precisely, this recursively traverses all children leftmost-first, then the direct elements
    leftmost-first.
  </li>
  <li>
    <code>"preorder"</code> (formerly <code>"naive_link"</code>): A left-to-right pre-ordering.
    Precisely, this traverses the direct elements leftmost-first, then recursively traverses the
    children leftmost-first.
  </li>
  <li>
    <code>"topological"</code> (formerly <code>"link"</code>): A topological ordering from the root
    down to the leaves. There is no left-to-right guarantee.
  </li>
</ul>

<p>Two depsets may only be merged if either both depsets have the same order, or one of them has
<code>"default"</code> order. In the latter case the resulting depset's order will be the same as
the other's order.

<p>Depsets may contain duplicate values but these will be suppressed when iterating (using
<a href="#to_list"><code>to_list()</code></a>). Duplicates may interfere with the ordering
semantics.

Ù
DirectoryExpander…
expandk
c
file7<a class="anchor" href="../builtins/File.html">File</a> The directory or file to expand.(list"—If the given <code>File</code> is a directory, this returns a list of <code>File</code>s recursively underneath the directory. Otherwise, this returns a list containing just the given <code>File</code> itself.íExpands directories created by <a href='../builtins/actions.html#declare_directory'><code>ctx.actions.declare_directory</code></a> during the execution phase. This is useful to expand directories in <a href='../builtins/Args.html#add_all.map_each'><code>map_each</code></a>.
‚
DottedVersionÔ

compare_tov
o
otherI<a class="anchor" href="../builtins/DottedVersion.html">DottedVersion</a>The other dotted version.(int"iCompares based on most significant (first) not-matching version component. So, for example, 1.2.3 < 1.2.4_A value representing a version with multiple components, separated by periods, such as 1.2.3.4.
ß
exec_resultº
return_codeint"ßThe return code returned after the execution of the program. 256 if the process was terminated by a time out; values larger than 128 indicate termination by a signal.
U
stderrstring"CThe content of the standard error output returned by the execution.O
stdoutstring"=The content of the standard output returned by the execution.∞A structure storing result of repository_ctx.execute() method. It contains the standard output stream content, the standard error stream content and the execution return code.

D
ExecGroupCollection-Stores exec groups available to a given rule.
Ñ
ExecGroupContextG

toolchainsToolchainContext"'Toolchains required for this exec group'Stores information about an exec group.
1
ExecTransitionFactoryan execution transition.
‰
ExpandedDirectory:
childrenlist"(Contains the files within the directory.9
	directoryFile"&The input directory that was expanded.XRepresents an expanded directory that makes the files within the it directly accessible.
≥
extension_metadataúReturn values of this type from a module extension's implementation function to provide metadata about the repositories generated by the extension to Bazel.
œ
FactsÅ
get™
U
key7<a class="anchor" href="../core/string.html">string</a>The key to look up.(
H
default7The value to return if <code>key</code> is not present."Noneunknown"MReturns the value for <code>key</code> if it exists, or <code>default</code>.¡User-provided data attached to a module extension that is persisted across reevaluations of
the extension.

This type supports dict-like access (e.g. `facts["key"]` and `facts.get("key")`) as well as
membership tests (e.g. `"key" in facts`). It does not support iteration or methods like
`keys()`, `items()`, or `len()`.

V
FeatureConfiguration>Class used to construct command lines from CROSSTOOL features.
·
Fileb
basenamestring"NThe base name of this file. This is the name of the file inside the directory.ü
dirnamestring"ãThe name of the directory containing this file. It's taken from <a href="#path">path</a> and is always relative to the execution directory.ü
	extensionstring"âThe file extension of this file, following (not including) the rightmost period. Empty string if the file's basename includes no periods.≈
is_directorybool"ÆReturns true if this is a directory. This reflects the type the file was declared as (i.e. ctx.actions.declare_directory), not its type on the filesystem, which might differ.S
	is_sourcebool"@Returns true if this is a source file, i.e. it is not generated.Ã

is_symlinkbool"∑Returns true if this was declared as a symlink. This reflects the type the file was declared as (i.e. ctx.actions.declare_symlink), not its type on the filesystem, which might differ.<
ownerLabel",A label of a target that produces this File.Ö
pathstring"ÙThe execution path of this file, relative to the workspace's execution directory. It consists of two parts, an optional first part called the <i>root</i> (see also the <a href="../builtins/root.html">root</a> module), and the second part which is the <code>short_path</code>. The root may be empty, which it usually is for non-generated files. For generated files it usually contains a configuration-specific path fragment that encodes things like the target CPU architecture that was used while building said file. Use the <code>short_path</code> for the path under which the file is mapped if it's in the runfiles of a binary.7
rootroot")The root beneath which this file resides.˝

short_pathstring"ÊThe path of this file relative to its root. This excludes the aforementioned <i>root</i>, i.e. configuration-specific fragments of the path. This is also the path under which the file is mapped if it's in the runfiles of a binary.ï
tree_relative_pathstring"ˆThe path of this file relative to the root of the ancestor's tree, if the ancestor's <a href="#is_directory">is_directory</a> field is true. <code>tree_relative_path</code> is only available for expanded files of a directory in an action command, i.e. <a href="../builtins/Args.html#add_all">Args.add_all()</a>. For other types of files, it is an error to access this field.≠This object is created during the analysis phase to represent a file or directory that will be read or written during the execution phase. It is not an open file handle, and cannot be used to directly read or write file contents. Rather, you use it to construct the action graph in a rule implementation function by passing it to action-creating functions. See the <a href='https://bazel.build/extending/rules#files'>Rules page</a> for more information.<p>When a <code>File</code> is passed to an <a href='../builtins/Args.html'><code>Args</code></a> object without using a <code>map_each</code> function, it is converted to a string by taking the value of its <code>path</code> field.
†
	fragmentsíA collection of configuration fragments available in the current rule implementation context. Access a specific fragment by its field name. For example, <code>ctx.fragments.java</code> <p>Only configuration fragments which are declared in the rule definition may be accessed in this collection.</p><p>See the <a href="../fragments.html">configuration fragment reference</a> for a list of available fragments and the <a href="https://bazel.build/extending/rules#configuration_fragments">rules documentation</a> for how to use them.
¥
java_annotation_processingi
	class_jarFile"VDeprecated: Please use <code>JavaInfo.java_outputs.generated_class_jar</code> instead.^
enabledbool"MDeprecated. Returns true if annotation processing was applied on this target.ü
processor_classnameslist"ÄDeprecated: Please use <code>JavaInfo.plugins</code> instead. Returns class names of annotation processors applied to this rule.†
processor_classpathdepset"ÄDeprecated: Please use <code>JavaInfo.plugins</code> instead. Returns a classpath of annotation processors applied to this rule.k

source_jarFile"WDeprecated: Please use <code>JavaInfo.java_outputs.generated_source_jar</code> instead.†
transitive_class_jarsdepset"Deprecated. Returns a transitive set of class file jars resulting from annotation processing of this rule and its dependencies.°
transitive_source_jarsdepset"Deprecated. Returns a transitive set of source archives resulting from annotation processing of this rule and its dependencies.RInformation about jars that are a result of annotation processing for a Java rule.
é!
Labelû
namestring"çThe name of the target referred to by this label. For instance:<br><pre class=language-python>Label("@@foo//pkg/foo:abc").name == "abc"</pre>›
packagestring"…The name of the package containing the target referred to by this label, without the repository name. For instance:<br><pre class=language-python>Label("@@repo//pkg/foo:abc").package == "pkg/foo"</pre>‹
relativeÑ
{
relName7<a class="anchor" href="../core/string.html">string</a>5The label that will be resolved relative to this one.(Label"»<b>Experimental</b>. This API is experimental and may change at any time. Please do not depend on it. It may be enabled on an experimental basis by setting <code>--+incompatible_enable_deprecated_label_apis</code> <br><strong>Deprecated.</strong> This method behaves surprisingly when used with an argument containing an apparent repo name. Prefer <a href="#same_package_label"><code>Label.same_package_label()</code></a>, <a href="../toplevel/native.html#package_relative_label"><code>native.package_relative_label()</code></a>, or <a href="#Label"><code>Label()</code></a> instead.<p>Resolves a label that is either absolute (starts with <code>//</code>) or relative to the current package. If this label is in a remote repository, the argument will be resolved relative to that repository. If the argument contains a repository name, the current label is ignored and the argument is returned as-is, except that the repository name is rewritten if it is in the current repository mapping. Reserved labels will also be returned as-is.<br>For example:<br><pre class=language-python>
Label("//foo/bar:baz").relative(":quux") == Label("//foo/bar:quux")
Label("//foo/bar:baz").relative("//wiz:quux") == Label("//wiz:quux")
Label("@repo//foo/bar:baz").relative("//wiz:quux") == Label("@repo//wiz:quux")
Label("@repo//foo/bar:baz").relative("//visibility:public") == Label("//visibility:public")
Label("@repo//foo/bar:baz").relative("@other//wiz:quux") == Label("@other//wiz:quux")
</pre><p>If the repository mapping passed in is <code>{'@other' : '@remapped'}</code>, then the following remapping will take place:<br><pre class=language-python>
Label("@repo//foo/bar:baz").relative("@other//wiz:quux") == Label("@remapped//wiz:quux")
</pre>Ù
	repo_namestring"ﬁThe canonical name of the repository containing the target referred to by this label, without any leading at-signs (<code>@</code>). For instance, <pre class=language-python>Label("@@foo//bar:baz").repo_name == "foo"</pre>Ÿ
same_package_labelt
k
target_name7<a class="anchor" href="../core/string.html">string</a>!The target name of the new label.(Label"MCreates a label in the same package as this label with the given target name.á
workspace_namestring"Ï<b>Experimental</b>. This API is experimental and may change at any time. Please do not depend on it. It may be enabled on an experimental basis by setting <code>--+incompatible_enable_deprecated_label_apis</code> <br><strong>Deprecated.</strong> The field name "workspace name" is a misnomer here; use the identically-behaving <a href="#repo_name"><code>Label.repo_name</code></a> instead.<p>The canonical name of the repository containing the target referred to by this label, without any leading at-signs (<code>@</code>). For instance, <pre class=language-python>Label("@@foo//bar:baz").workspace_name == "foo"</pre>É
workspace_rootstring"ËReturns the execution root for the repository containing the target referred to by this label, relative to the execroot. For instance:<br><pre class=language-python>Label("@repo//pkg/foo:abc").workspace_root == "external/repo"</pre>¡A BUILD target identifier.<p>For every <code>Label</code> instance <code>l</code>, the string representation <code>str(l)</code> has the property that <code>Label(str(l)) == l</code>, regardless of where the <code>Label()</code> call occurs.<p>When passed as positional arguments to <code>print()</code> or <code>fail()</code>, <code>Label</code> use a string representation optimized for human readability instead. This representation uses an <a href="/external/overview#apparent-repo-name">apparent repository name</a> from the perspective of the main repository if possible.
¬
LateBoundDefault≠Represents a late-bound default attribute value of type 'Label'. The value of a LateBoundDefault is only resolvable in the context of a rule implementation function, and depends on the current build configuration. For example, a LateBoundDefault might represent the Label of the java toolchain in the current build configuration. <p>See <a href="../globals/bzl.html#configuration_field">configuration_field</a> for example usage.
á

LibraryToLink\

alwayslinkbool"HWhether to link the static library/objects in the --whole_archive block.Æ
dynamic_libraryFile"î<code>Artifact</code> of dynamic library to be linked. Always used for runtime and used for linking if <code>interface_library</code> is not passed.S
interface_libraryFile"8<code>Artifact</code> of interface library to be linked.U
lto_bitcode_filessequence"6<code>List</code> of LTO bitcode files in the library.F
objectssequence"1<code>List</code> of object files in the library.]
pic_lto_bitcode_filessequence":<code>List</code> of pic LTO bitcode files in the library.N
pic_objectssequence"5<code>List</code> of pic object files in the library.U
pic_static_libraryFile"9<code>Artifact</code> of pic static library to be linked.∑
 resolved_symlink_dynamic_libraryFile"åThe resolved <code>Artifact</code> of the dynamic library to be linked if <code>dynamic_library</code> is a symlink, otherwise this is None.Ω
"resolved_symlink_interface_libraryFile"êThe resolved <code>Artifact</code> of the interface library to be linked if <code>interface_library</code> is a symlink, otherwise this is None.M
static_libraryFile"5<code>Artifact</code> of static library to be linked.$A library the user can link against.
å
LicenseÄThis API is deprecated and will be removed. Please do not depend on it. This object represents the value of a license attribute.
Ã
LinkerInput]
additional_inputssequence">Returns the depset of additional inputs, e.g.: linker scripts.}
	librariessequence"fReturns the depset of <code>LibraryToLink</code>. May return a list but this is deprecated. See #8118.6
ownerLabel"&Returns the owner of this LinkerInput.S
user_link_flagssequence"6Returns the list of user link flags passed as strings.REither libraries, flags or other files that may be passed to the linker as inputs.
Æ
LinkingContext=
linker_inputsdepset"$Returns the depset of linker inputs.]Immutable store of information needed for C++ linking that is aggregated across dependencies.
™
macro†A callable Starlark value representing a symbolic macro; in other words, the return value of
<a href="../globals/bzl.html#macro"><code>macro()</code></a>. Invoking this value during package
construction time will instantiate the macro, and cause the macro's implementation function to be
evaluated (in a separate context, different from the context in which the macro value was invoked),
in most cases causing targets to be added to the package's target set. For more information, see
<a href="https://bazel.build/extending/macros">Macros</a>.

ò
mapped_rootP
pathstring"@Returns the relative path from the exec root to the actual root.7A root for files that have been subject to path mapping
⁄ß

module_ctxÓ
downloadÃ
∫
urlÄ<a class="anchor" href="../core/string.html">string</a>; or Iterable of <a class="anchor" href="../core/string.html">string</a>s.List of mirror URLs referencing the same file.(
Ä
output±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>>path to the output file, relative to the repository directory."''
È
sha2567<a class="anchor" href="../core/string.html">string</a>°The expected SHA-256 hash of the file downloaded. This must match the SHA-256 hash of the file downloaded. It is a security risk to omit the SHA-256 as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping. If provided, the repository cache will first be checked for a file with the given hash; a download will only be attempted if the file was not found in the cache. After a successful download, the file will be added to the cache.
"''
à

executable3<a class="anchor" href="../core/bool.html">bool</a>>Set the executable flag on the created file, false by default."False
´

allow_fail3<a class="anchor" href="../core/bool.html">bool</a>aIf set, indicate the error in the return value instead of raising an error for failed downloads.
"False
ë
canonical_id7<a class="anchor" href="../core/string.html">string</a>√If set, restrict cache hits to those cases where the file was added to the cache with the same canonical id. By default caching uses the checksum (<code>sha256</code> or <code>integrity</code>).
"''
ç
auth3<a class="anchor" href="../core/dict.html">dict</a>LAn optional dict specifying authentication information for some of the URLs."{}
z
headers3<a class="anchor" href="../core/dict.html">dict</a>6An optional dict specifying http headers for all URLs."{}
Ü
	integrity7<a class="anchor" href="../core/string.html">string</a>ªExpected checksum of the file downloaded, in Subresource Integrity format. This must match the checksum of the file downloaded. It is a security risk to omit the checksum as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping. If provided, the repository cache will first be checked for a file with the given checksum; a download will only be attempted if the file was not found in the cache. After a successful download, the file will be added to the cache.
"''
≤
block3<a class="anchor" href="../core/bool.html">bool</a>ÌIf set to false, the call returns immediately and instead of the regular return value, it returns a token with one single method, wait(), which blocks until the download is finished and returns the usual return value or throws as usual.
"Trueunknown"íDownloads a file to the output path for the provided url and returns a struct containing <code>success</code>, a flag which is <code>true</code> if the download completed successfully, and if successful, a hash of the file with the fields <code>sha256</code> and <code>integrity</code>. When <code>sha256</code> or <code>integrity</code> is user specified, setting an explicit <code>canonical_id</code> is highly recommended. e.g. <a href='/rules/lib/repo/cache#get_default_canonical_id'><code>get_default_canonical_id</code></a>
à"
download_and_extractÃ
∫
urlÄ<a class="anchor" href="../core/string.html">string</a>; or Iterable of <a class="anchor" href="../core/string.html">string</a>s.List of mirror URLs referencing the same file.(
¢
output±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>`Path to the directory where the archive will be unpacked, relative to the repository directory.
"''
È
sha2567<a class="anchor" href="../core/string.html">string</a>°The expected SHA-256 hash of the file downloaded. This must match the SHA-256 hash of the file downloaded. It is a security risk to omit the SHA-256 as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping. If provided, the repository cache will first be checked for a file with the given hash; a download will only be attempted if the file was not found in the cache. After a successful download, the file will be added to the cache.
"''
ô
type7<a class="anchor" href="../core/string.html">string</a>”The archive type of the downloaded file. By default, the archive type is determined from the file extension of the URL. If the file has no extension, you can explicitly specify either "zip", "jar", "war", "aar", "nupkg", "whl", "tar", "tar.gz", "tgz", "tar.xz", "txz", ".tar.zst", ".tzst", "tar.bz2", ".tbz", ".ar", ".deb", or ".7z" here.
"''
‘
strip_prefix7<a class="anchor" href="../core/string.html">string</a>ÜA directory prefix to strip from the extracted files. Many archives contain a
top-level directory that contains all files in the archive. Instead of needing to
specify this prefix over and over in the <code>build_file</code>, this field can
be used to strip it from extracted files.

<p>For compatibility, this parameter may also be used under the deprecated name
<code>stripPrefix</code>.
"''
´

allow_fail3<a class="anchor" href="../core/bool.html">bool</a>aIf set, indicate the error in the return value instead of raising an error for failed downloads.
"False
ë
canonical_id7<a class="anchor" href="../core/string.html">string</a>√If set, restrict cache hits to those cases where the file was added to the cache with the same canonical id. By default caching uses the checksum
(<code>sha256</code> or <code>integrity</code>).
"''
ç
auth3<a class="anchor" href="../core/dict.html">dict</a>LAn optional dict specifying authentication information for some of the URLs."{}
z
headers3<a class="anchor" href="../core/dict.html">dict</a>6An optional dict specifying http headers for all URLs."{}
Ü
	integrity7<a class="anchor" href="../core/string.html">string</a>ªExpected checksum of the file downloaded, in Subresource Integrity format. This must match the checksum of the file downloaded. It is a security risk to omit the checksum as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping. If provided, the repository cache will first be checked for a file with the given checksum; a download will only be attempted if the file was not found in the cache. After a successful download, the file will be added to the cache. "''
©
rename_files3<a class="anchor" href="../core/dict.html">dict</a>ﬂAn optional dict specifying files to rename during the extraction. Archive entries with names exactly matching a key will be renamed to the value, prior to any directory prefix adjustment. This can be used to extract archives that contain non-Unicode filenames, or which have files that would extract to the same path on case-insensitive filesystems.
"{}struct"†Downloads a file to the output path for the provided url, extracts it, and returns a struct containing <code>success</code>, a flag which is <code>true</code> if the download completed successfully, and if successful, a hash of the file with the fields <code>sha256</code> and <code>integrity</code>. When <code>sha256</code> or <code>integrity</code> is user specified, setting an explicit <code>canonical_id</code> is highly recommended. e.g. <a href='/rules/lib/repo/cache#get_default_canonical_id'><code>get_default_canonical_id</code></a>
·	
execute‰
õ
	arguments7<a class="anchor" href="../core/list.html">sequence</a>SList of arguments, the first element should be the path to the program to execute.
(
á
timeout1<a class="anchor" href="../core/int.html">int</a>DMaximum duration of the command in seconds (default is 600 seconds)."600
ÿ
environment3<a class="anchor" href="../core/dict.html">dict</a>èForce some environment variables to be set to be passed to the process. The value can be <code>None</code> to remove the environment variable.
"{}
{
quiet3<a class="anchor" href="../core/bool.html">bool</a>7If stdout and stderr should be printed to the terminal."True
‘
working_directory7<a class="anchor" href="../core/string.html">string</a>ÅWorking directory for command execution.
Can be relative to the repository root or absolute.
The default is the repository root.
"""exec_result"ÓExecutes the command given by the list of arguments. The execution time of the command is limited by <code>timeout</code> (in seconds, default 600 seconds). This method returns an <code>exec_result</code> structure containing the output of the command. The <code>environment</code> map can be used to override some environment variables to be passed to the process.
å 
extension_metadataæ
˚	
root_module_direct_deps≈<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>ëThe names of the repositories that the extension considers to be direct dependencies of the root module. If the root module imports additional repositories or does not import all of these repositories via <a href="../globals/module.html#use_repo"><code>use_repo</code></a>, Bazel will print a warning when the extension is evaluated, instructing the user to run <code>bazel mod tidy</code> to fix the <code>use_repo</code> calls automatically. <p>If one of <code>root_module_direct_deps</code> and will print a warning and a fixup command when the extension is evaluated.<p>If one of <code>root_module_direct_deps</code> and <code>root_module_direct_dev_deps</code> is specified, the other has to be as well. The lists specified by these two parameters must be disjoint.<p>Exactly one of <code>root_module_direct_deps</code> and <code>root_module_direct_dev_deps</code> can be set to the special value <code>"all"</code>, which is treated as if a list with the names of all repositories generated by the extension was specified as the value."None
í

root_module_direct_dev_deps≈<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>§The names of the repositories that the extension considers to be direct dev dependencies of the root module. If the root module imports additional repositories or does not import all of these repositories via <a href="../globals/module.html#use_repo"><code>use_repo</code></a> on an extension proxy created with <code><a href="../globals/module.html#use_extension">use_extension</a>(..., dev_dependency = True)</code>, Bazel will print a warning when the  extension is evaluated, instructing the user to run <code>bazel mod tidy</code> to fix the <code>use_repo</code> calls automatically. <p>If one of <code>root_module_direct_deps</code> and <code>root_module_direct_dev_deps</code> is specified, the other has to be as well. The lists specified by these two parameters must be disjoint.<p>Exactly one of <code>root_module_direct_deps</code> and <code>root_module_direct_dev_deps</code> can be set to the special value <code>"all"</code>, which is treated as if a list with the names of all repositories generated by the extension was specified as the value."None
¿
reproducible3<a class="anchor" href="../core/bool.html">bool</a>tStates that this module extension ensures complete reproducibility, thereby it should not be stored in the lockfile."False
—
factso<a class="anchor" href="../core/dict.html">dict</a> of <a class="anchor" href="../core/string.html">string</a>s“A JSON-like dict that is made available to future executions of this extension via
the `module_ctx.facts` property.
This is useful for extensions that want to preserve universally true facts such as
the hashes of artifacts in an immutable repository.

Bazel may shallowly merge multiple facts dicts returned by different versions of the
extension in order to resolve merge conflicts on the MODULE.bazel.lock file, as if
by applying the `dict.update()` method or the `|` operator in Starlark. Extensions
should use facts for key-value storage only and ensure that the key uniquely
determines the value, although perhaps only via additional information and network
access. An extension can opt out of this merging by providing a dict with a single,
fixed top-level key and an arbitrary value.

Note that the value provided here may be read back by a different version of the
extension, so either include a version number or use a schema that is unlikely to
result in ambiguities.
"{}extension_metadata"¥Constructs an opaque object that can be returned from the module extension's implementation function to provide metadata about the repositories generated by the extension to Bazel.’
extractò
ë
archive±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>Ppath to the archive that will be unpacked, relative to the repository directory.(
°
output±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>_path to the directory where the archive will be unpacked, relative to the repository directory."''
‘
strip_prefix7<a class="anchor" href="../core/string.html">string</a>Üa directory prefix to strip from the extracted files. Many archives contain a
top-level directory that contains all files in the archive. Instead of needing to
specify this prefix over and over in the <code>build_file</code>, this field can be
used to strip it from extracted files.

<p>For compatibility, this parameter may also be used under the deprecated name
<code>stripPrefix</code>.
"''
®
rename_files3<a class="anchor" href="../core/dict.html">dict</a>ﬁAn optional dict specifying files to rename during the extraction. Archive entries with names exactly matching a key will be renamed to the value, prior to any directory prefix adjustment. This can be used to extract archives that contain non-Unicode filenames, or which have files that would extract to the same path on case-insensitive filesystems."{}
—
watch_archive7<a class="anchor" href="../core/string.html">string</a>˛whether to <a href="#watch">watch</a> the archive file. Can be the string 'yes', 'no', or 'auto'. Passing 'yes' is equivalent to immediately invoking the <a href="#watch"><code>watch()</code></a> method; passing 'no' does not attempt to watch the file; passing 'auto' will only attempt to watch the file when it is legal to do so (see <code>watch()</code> docs for more information."'auto'NoneType"/Extract an archive to the repository directory.Œ
factsFacts"ΩThe JSON-like dict returned by a previous execution of this extension in the `facts`
parameter of [`extension_metadata`](../builtins/module_ctx#extension_metadata) or else
`{}`.
This is useful for extensions that want to preserve universally true facts such as the
hashes of artifacts in an immutable repository.
Note that the returned value may have been created by a different version of the
extension, which may have used a different schema.
â
file∑
ˇ
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>APath of the file to create, relative to the repository directory.(
|
content7<a class="anchor" href="../core/string.html">string</a>4The content of the file to create, empty by default."''
Ü

executable3<a class="anchor" href="../core/bool.html">bool</a>=Set the executable flag on the created file, true by default."True
°
legacy_utf83<a class="anchor" href="../core/bool.html">bool</a>VNo-op. This parameter is deprecated and will be removed in a future version of Bazel.
"FalseNoneType"GGenerates a file in the repository directory with the provided content.™
getenvè
h
name7<a class="anchor" href="../core/string.html">string</a>%Name of desired environment variable.(
ö
defaultM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>:Default value to return if <code>name</code> is not found."Nonestring"çReturns the value of an environment variable <code>name</code> as a string if exists, or <code>default</code> if it doesn't. <p>When building incrementally, any change to the value of the variable named by <code>name</code> will cause this repository to be re-fetched.
æ
is_dev_dependencyz
r
tagbazel_module_tagWA tag obtained from <a href="../builtins/bazel_module.html#tags">bazel_module.tags</a>.(bool"¨Returns whether the given tag was specified on the result of a <a href="../globals/module.html#use_extension">use_extension</a> call with <code>devDependency = True</code>.˚
moduleslist"ÈA list of all the Bazel modules in the external dependency graph that use this module extension, each of which is a <a href="../builtins/bazel_module.html">bazel_module</a> object that exposes all the tags it specified for this extension. The iteration order of this dictionary is guaranteed to be the same as breadth-first search starting from the root module.D
osrepository_os"/A struct to access information from the system.î
path•
ú
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>^<code>string</code>, <code>Label</code> or <code>path</code> from which to create a path from.(path"„Returns a path from a string, label, or path. If this context is a <code>repository_ctx</code>, a relative path will resolve relative to the repository directory. If it is a <code>module_ctx</code>, a relative path will resolve relative to a temporary working directory for this module extension. If the path is a label, it will resolve to the path of the corresponding file. Note that remote repositories and module extensions are executed during the analysis phase and thus cannot depends on a target result (the label should point to a non-generated file). If path is a path, it will return that path as is.
Â
read¨
‹
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>Path of the file to read from.(
¬
watch7<a class="anchor" href="../core/string.html">string</a>˜Whether to <a href="#watch">watch</a> the file. Can be the string 'yes', 'no', or 'auto'. Passing 'yes' is equivalent to immediately invoking the <a href="#watch"><code>watch()</code></a> method; passing 'no' does not attempt to watch the file; passing 'auto' will only attempt to watch the file when it is legal to do so (see <code>watch()</code> docs for more information.
"'auto'string".Reads the content of a file on the filesystem.Ü
report_progressú
è
status7<a class="anchor" href="../core/string.html">string</a>H<code>string</code> describing the current status of the fetch progress."''NoneType"TUpdates the progress status for the fetching of this repository or module extension.p
"root_module_has_non_dev_dependencybool"DWhether the root module uses this extension as a non-dev dependency.ã	
watchÂ
ÿ
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>Path of the file to watch.(NoneType"ôTells Bazel to watch for changes to the given path, whether or not it exists, or whether it's a file or a directory. Any changes to the file or directory will invalidate this repository or module extension, and cause it to be refetched or re-evaluated next time.<p>"Changes" include changes to the contents of the file (if the path is a file); if the path was a file but is now a directory, or vice versa; and if the path starts or stops existing. Notably, this does <em>not</em> include changes to any files under the directory if the path is a directory. For that, use <a href="path.html#readdir"><code>path.readdir()</code></a> instead.<p>Note that attempting to watch paths inside the repo currently being fetched, or inside the working directory of the current module extension, will result in an error. A module extension attempting to watch a path outside the current Bazel workspace will also result in an error.
Ó
whichj
b
program7<a class="anchor" href="../core/string.html">string</a>Program to find in the path.(path"yReturns the <code>path</code> of the corresponding program or <code>None</code> if there is no such program in the path.
¸The context of the module extension containing helper functions and information about pertinent tags across the dependency graph. You get a module_ctx object as an argument to the <code>implementation</code> function when you create a module extension.
¡
path=
basenamestring")A string giving the basename of the file.`
dirnamepath"OThe parent directory of this file, or None if this file does not have a parent.π
existsbool"®Returns true if the file or directory denoted by this path exists.<p>Note that accessing this field does <em>not</em> cause the path to be watched. If you'd like the repo rule or module extension to be sensitive to the path's existence, use the <code>watch()</code> method on the context object.
€
	get_child
w
*relative_paths`Zero or more relative path strings to append to this path with path separators added as needed.
(0path"MReturns the path obtained by joining this path with the given relative paths.º
is_dirbool"´Returns true if this path points to a directory.<p>Note that accessing this field does <em>not</em> cause the path to be watched. If you'd like the repo rule or module extension to be sensitive to whether the path is a directory or a file, use the <code>watch()</code> method on the context object.
ˇ
readdir˝
Ù
watch7<a class="anchor" href="../core/string.html">string</a>©whether Bazel should watch the list of entries in this directory and refetch the repository or re-evaluate the module extension next time when any changes are detected. Changes to detect include entry creation, deletion, and renaming. Note that this doesn't watch the <em>contents</em> of any entries in the directory.<p>Can be the string 'yes', 'no', or 'auto'. If set to 'auto', Bazel will only watch this directory when it is legal to do so (see <a href="repository_ctx.html#watch"><code>repository_ctx.watch()</code></a> docs for more information).
"'auto'list"tReturns the list of entries in the directory denoted by this path. Each entry is a <code>path</code> object itself.
|
realpathpath"jReturns the canonical path for this path by repeatedly replacing all symbolic links with their referents.
?A structure representing a file to be used inside a repository.
÷
propagation_ctxî
attrstruct"ÉA struct to access only the public parameters of the aspect. The keys and values of the struct are the parameters names and values.S
rule StarlarkAspectPropagationRuleApi")Allows access to the details of the rule.÷A context object that is passed to the <code>propagation_predicate</code>, <code>attr_aspects</code> and <code>toolchains_aspects</code> functions of aspects. It provides access to the information needed to determine whether the aspect should be propagated to the target and what attributes or toolchain types it should be propagated to next.
‰
Provider◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.
k
repo_metadataZSee <a href="repository_ctx#repo_metadata"><code>repository_ctx.repo_metadata</code></a>.

ñ∂
repository_ctxã
attr	structure"xA struct to access the values of the attributes. The values are provided by the user (if not, a default value is used).
˙
deleteÚ
È
paths<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/path.html">path</a>jPath of the file to delete, relative to the repository directory, or absolute. Can be a path or a string.
(bool"{Deletes a file or a directory. Returns a bool, indicating whether the file or directory was actually deleted by this call.
Ó
downloadÃ
∫
urlÄ<a class="anchor" href="../core/string.html">string</a>; or Iterable of <a class="anchor" href="../core/string.html">string</a>s.List of mirror URLs referencing the same file.(
Ä
output±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>>path to the output file, relative to the repository directory."''
È
sha2567<a class="anchor" href="../core/string.html">string</a>°The expected SHA-256 hash of the file downloaded. This must match the SHA-256 hash of the file downloaded. It is a security risk to omit the SHA-256 as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping. If provided, the repository cache will first be checked for a file with the given hash; a download will only be attempted if the file was not found in the cache. After a successful download, the file will be added to the cache.
"''
à

executable3<a class="anchor" href="../core/bool.html">bool</a>>Set the executable flag on the created file, false by default."False
´

allow_fail3<a class="anchor" href="../core/bool.html">bool</a>aIf set, indicate the error in the return value instead of raising an error for failed downloads.
"False
ë
canonical_id7<a class="anchor" href="../core/string.html">string</a>√If set, restrict cache hits to those cases where the file was added to the cache with the same canonical id. By default caching uses the checksum (<code>sha256</code> or <code>integrity</code>).
"''
ç
auth3<a class="anchor" href="../core/dict.html">dict</a>LAn optional dict specifying authentication information for some of the URLs."{}
z
headers3<a class="anchor" href="../core/dict.html">dict</a>6An optional dict specifying http headers for all URLs."{}
Ü
	integrity7<a class="anchor" href="../core/string.html">string</a>ªExpected checksum of the file downloaded, in Subresource Integrity format. This must match the checksum of the file downloaded. It is a security risk to omit the checksum as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping. If provided, the repository cache will first be checked for a file with the given checksum; a download will only be attempted if the file was not found in the cache. After a successful download, the file will be added to the cache.
"''
≤
block3<a class="anchor" href="../core/bool.html">bool</a>ÌIf set to false, the call returns immediately and instead of the regular return value, it returns a token with one single method, wait(), which blocks until the download is finished and returns the usual return value or throws as usual.
"Trueunknown"íDownloads a file to the output path for the provided url and returns a struct containing <code>success</code>, a flag which is <code>true</code> if the download completed successfully, and if successful, a hash of the file with the fields <code>sha256</code> and <code>integrity</code>. When <code>sha256</code> or <code>integrity</code> is user specified, setting an explicit <code>canonical_id</code> is highly recommended. e.g. <a href='/rules/lib/repo/cache#get_default_canonical_id'><code>get_default_canonical_id</code></a>
à"
download_and_extractÃ
∫
urlÄ<a class="anchor" href="../core/string.html">string</a>; or Iterable of <a class="anchor" href="../core/string.html">string</a>s.List of mirror URLs referencing the same file.(
¢
output±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>`Path to the directory where the archive will be unpacked, relative to the repository directory.
"''
È
sha2567<a class="anchor" href="../core/string.html">string</a>°The expected SHA-256 hash of the file downloaded. This must match the SHA-256 hash of the file downloaded. It is a security risk to omit the SHA-256 as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping. If provided, the repository cache will first be checked for a file with the given hash; a download will only be attempted if the file was not found in the cache. After a successful download, the file will be added to the cache.
"''
ô
type7<a class="anchor" href="../core/string.html">string</a>”The archive type of the downloaded file. By default, the archive type is determined from the file extension of the URL. If the file has no extension, you can explicitly specify either "zip", "jar", "war", "aar", "nupkg", "whl", "tar", "tar.gz", "tgz", "tar.xz", "txz", ".tar.zst", ".tzst", "tar.bz2", ".tbz", ".ar", ".deb", or ".7z" here.
"''
‘
strip_prefix7<a class="anchor" href="../core/string.html">string</a>ÜA directory prefix to strip from the extracted files. Many archives contain a
top-level directory that contains all files in the archive. Instead of needing to
specify this prefix over and over in the <code>build_file</code>, this field can
be used to strip it from extracted files.

<p>For compatibility, this parameter may also be used under the deprecated name
<code>stripPrefix</code>.
"''
´

allow_fail3<a class="anchor" href="../core/bool.html">bool</a>aIf set, indicate the error in the return value instead of raising an error for failed downloads.
"False
ë
canonical_id7<a class="anchor" href="../core/string.html">string</a>√If set, restrict cache hits to those cases where the file was added to the cache with the same canonical id. By default caching uses the checksum
(<code>sha256</code> or <code>integrity</code>).
"''
ç
auth3<a class="anchor" href="../core/dict.html">dict</a>LAn optional dict specifying authentication information for some of the URLs."{}
z
headers3<a class="anchor" href="../core/dict.html">dict</a>6An optional dict specifying http headers for all URLs."{}
Ü
	integrity7<a class="anchor" href="../core/string.html">string</a>ªExpected checksum of the file downloaded, in Subresource Integrity format. This must match the checksum of the file downloaded. It is a security risk to omit the checksum as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping. If provided, the repository cache will first be checked for a file with the given checksum; a download will only be attempted if the file was not found in the cache. After a successful download, the file will be added to the cache. "''
©
rename_files3<a class="anchor" href="../core/dict.html">dict</a>ﬂAn optional dict specifying files to rename during the extraction. Archive entries with names exactly matching a key will be renamed to the value, prior to any directory prefix adjustment. This can be used to extract archives that contain non-Unicode filenames, or which have files that would extract to the same path on case-insensitive filesystems.
"{}struct"†Downloads a file to the output path for the provided url, extracts it, and returns a struct containing <code>success</code>, a flag which is <code>true</code> if the download completed successfully, and if successful, a hash of the file with the fields <code>sha256</code> and <code>integrity</code>. When <code>sha256</code> or <code>integrity</code> is user specified, setting an explicit <code>canonical_id</code> is highly recommended. e.g. <a href='/rules/lib/repo/cache#get_default_canonical_id'><code>get_default_canonical_id</code></a>
·	
execute‰
õ
	arguments7<a class="anchor" href="../core/list.html">sequence</a>SList of arguments, the first element should be the path to the program to execute.
(
á
timeout1<a class="anchor" href="../core/int.html">int</a>DMaximum duration of the command in seconds (default is 600 seconds)."600
ÿ
environment3<a class="anchor" href="../core/dict.html">dict</a>èForce some environment variables to be set to be passed to the process. The value can be <code>None</code> to remove the environment variable.
"{}
{
quiet3<a class="anchor" href="../core/bool.html">bool</a>7If stdout and stderr should be printed to the terminal."True
‘
working_directory7<a class="anchor" href="../core/string.html">string</a>ÅWorking directory for command execution.
Can be relative to the repository root or absolute.
The default is the repository root.
"""exec_result"ÓExecutes the command given by the list of arguments. The execution time of the command is limited by <code>timeout</code> (in seconds, default 600 seconds). This method returns an <code>exec_result</code> structure containing the output of the command. The <code>environment</code> map can be used to override some environment variables to be passed to the process.
’
extractò
ë
archive±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>Ppath to the archive that will be unpacked, relative to the repository directory.(
°
output±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>_path to the directory where the archive will be unpacked, relative to the repository directory."''
‘
strip_prefix7<a class="anchor" href="../core/string.html">string</a>Üa directory prefix to strip from the extracted files. Many archives contain a
top-level directory that contains all files in the archive. Instead of needing to
specify this prefix over and over in the <code>build_file</code>, this field can be
used to strip it from extracted files.

<p>For compatibility, this parameter may also be used under the deprecated name
<code>stripPrefix</code>.
"''
®
rename_files3<a class="anchor" href="../core/dict.html">dict</a>ﬁAn optional dict specifying files to rename during the extraction. Archive entries with names exactly matching a key will be renamed to the value, prior to any directory prefix adjustment. This can be used to extract archives that contain non-Unicode filenames, or which have files that would extract to the same path on case-insensitive filesystems."{}
—
watch_archive7<a class="anchor" href="../core/string.html">string</a>˛whether to <a href="#watch">watch</a> the archive file. Can be the string 'yes', 'no', or 'auto'. Passing 'yes' is equivalent to immediately invoking the <a href="#watch"><code>watch()</code></a> method; passing 'no' does not attempt to watch the file; passing 'auto' will only attempt to watch the file when it is legal to do so (see <code>watch()</code> docs for more information."'auto'NoneType"/Extract an archive to the repository directory.â
file∑
ˇ
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>APath of the file to create, relative to the repository directory.(
|
content7<a class="anchor" href="../core/string.html">string</a>4The content of the file to create, empty by default."''
Ü

executable3<a class="anchor" href="../core/bool.html">bool</a>=Set the executable flag on the created file, true by default."True
°
legacy_utf83<a class="anchor" href="../core/bool.html">bool</a>VNo-op. This parameter is deprecated and will be removed in a future version of Bazel.
"FalseNoneType"GGenerates a file in the repository directory with the provided content.™
getenvè
h
name7<a class="anchor" href="../core/string.html">string</a>%Name of desired environment variable.(
ö
defaultM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>:Default value to return if <code>name</code> is not found."Nonestring"çReturns the value of an environment variable <code>name</code> as a string if exists, or <code>default</code> if it doesn't. <p>When building incrementally, any change to the value of the variable named by <code>name</code> will cause this repository to be re-fetched.
¯
namestring"ÁThe canonical name of the external repository created by this rule. This name is guaranteed to be unique among all external repositories, but its exact format is not specified. Use <a href='#original_name'><code>original_name</code></a> instead to get the name that was originally specified as the <code>name</code> when this repository rule was instantiated.∏
original_namestring"ûThe name that was originally specified as the <code>name</code> attribute when this repository rule was instantiated. This name is not necessarily unique among external repositories. Use <a href='#name'><code>name</code></a> instead to get the canonical name of the external repository.D
osrepository_os"/A struct to access information from the system.˜	
patch∂
’

patch_file±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>êThe patch file to apply, it can be label, relative path or absolute path. If it's a relative path, it will resolve to the repository directory.
(
Ä
strip1<a class="anchor" href="../core/int.html">int</a>AStrip the specified number of leading components from file names."0
Œ
watch_patch7<a class="anchor" href="../core/string.html">string</a>˝Whether to <a href="#watch">watch</a> the patch file. Can be the string 'yes', 'no', or 'auto'. Passing 'yes' is equivalent to immediately invoking the <a href="#watch"><code>watch()</code></a> method; passing 'no' does not attempt to watch the file; passing 'auto' will only attempt to watch the file when it is legal to do so (see <code>watch()</code> docs for more information.
"'auto'NoneType"¥Apply a patch file to the root directory of external repository. The patch file should be a standard <a href="https://en.wikipedia.org/wiki/Diff#Unified_format"> unified diff format</a> file. The Bazel-native patch implementation doesn't support fuzz match and binary patch like the patch command line tool.
î
path•
ú
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>^<code>string</code>, <code>Label</code> or <code>path</code> from which to create a path from.(path"„Returns a path from a string, label, or path. If this context is a <code>repository_ctx</code>, a relative path will resolve relative to the repository directory. If it is a <code>module_ctx</code>, a relative path will resolve relative to a temporary working directory for this module extension. If the path is a label, it will resolve to the path of the corresponding file. Note that remote repositories and module extensions are executed during the analysis phase and thus cannot depends on a target result (the label should point to a non-generated file). If path is a path, it will return that path as is.
Â
read¨
‹
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>Path of the file to read from.(
¬
watch7<a class="anchor" href="../core/string.html">string</a>˜Whether to <a href="#watch">watch</a> the file. Can be the string 'yes', 'no', or 'auto'. Passing 'yes' is equivalent to immediately invoking the <a href="#watch"><code>watch()</code></a> method; passing 'no' does not attempt to watch the file; passing 'auto' will only attempt to watch the file when it is legal to do so (see <code>watch()</code> docs for more information.
"'auto'string".Reads the content of a file on the filesystem.≠
renameÃ
ô
src±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>\The path of the existing file or directory to rename, relative
to the repository directory.
(
£
dst±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>fThe new name to which the file or directory will be renamed to,
relative to the repository directory.
(NoneType"”Renames the file or directory from <code>src</code> to <code>dst</code>. Parent directories are created as needed. Fails if the destination path
already exists. Both paths must be located within the repository.
å	
repo_metadataÍ
Â
reproducible3<a class="anchor" href="../core/bool.html">bool</a>òStates that this repo can be reproducibly refetched; that is, if it were fetched another time with exactly the same input attributes, repo rule definition, watched files and environment variables, etc., then exactly the same output would be produced. This property needs to hold even if other untracked conditions change, such as information from the internet, the path of the workspace root, output from running arbitrary executables, etc. If set to True, this allows the fetched repo contents to be cached across workspaces. <p>Note that setting this to True does not guarantee caching in the repo contents cache; for example, local repo rules are never cached.
"False

attrs_for_reproducibility3<a class="anchor" href="../core/dict.html">dict</a>ôIf <code>reproducible</code> is False, this can be specified to tell Bazel which attributes of the original repo rule to change to make it reproducible.
"{}repo_metadata"çConstructs an opaque object that can be returned from the repo rule's implementation function to provide metadata about its reproducibility.
Ü
report_progressú
è
status7<a class="anchor" href="../core/string.html">string</a>H<code>string</code> describing the current status of the fetch progress."''NoneType"TUpdates the progress status for the fetching of this repository or module extension.ë
symlinkﬂ
Í
target±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>*The path that the symlink should point to.(
Â
	link_name±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>"The path of the symlink to create.(NoneType"$Creates a symlink on the filesystem.ß
template…	
ˇ
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>APath of the file to create, relative to the repository directory.(
‹
template±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>Path to the template file.(
|
substitutions3<a class="anchor" href="../core/dict.html">dict</a>2Substitutions to make when expanding the template."{}
Ü

executable3<a class="anchor" href="../core/bool.html">bool</a>=Set the executable flag on the created file, true by default."True
‘
watch_template7<a class="anchor" href="../core/string.html">string</a>ÄWhether to <a href="#watch">watch</a> the template file. Can be the string 'yes', 'no', or 'auto'. Passing 'yes' is equivalent to immediately invoking the <a href="#watch"><code>watch()</code></a> method; passing 'no' does not attempt to watch the file; passing 'auto' will only attempt to watch the file when it is legal to do so (see <code>watch()</code> docs for more information.
"'auto'NoneType"ŒGenerates a new file using a <code>template</code>. Every occurrence in <code>template</code> of a key of <code>substitutions</code> will be replaced by the corresponding value. The result is written in <code>path</code>. An optional <code>executable</code> argument (default to true) can be set to turn on or off the executable bit.
ã	
watchÂ
ÿ
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>Path of the file to watch.(NoneType"ôTells Bazel to watch for changes to the given path, whether or not it exists, or whether it's a file or a directory. Any changes to the file or directory will invalidate this repository or module extension, and cause it to be refetched or re-evaluated next time.<p>"Changes" include changes to the contents of the file (if the path is a file); if the path was a file but is now a directory, or vice versa; and if the path starts or stops existing. Notably, this does <em>not</em> include changes to any files under the directory if the path is a directory. For that, use <a href="path.html#readdir"><code>path.readdir()</code></a> instead.<p>Note that attempting to watch paths inside the repo currently being fetched, or inside the working directory of the current module extension, will result in an error. A module extension attempting to watch a path outside the current Bazel workspace will also result in an error.
⁄

watch_treeÔ
‚
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>$Path of the directory tree to watch.(NoneType"ŸTells Bazel to watch for changes to any files or directories transitively under the given path. Any changes to the contents of files, the existence of files or directories, file names or directory names, will cause this repo to be refetched.<p>Note that attempting to watch paths inside the repo currently being fetched will result in an error.
Ó
whichj
b
program7<a class="anchor" href="../core/string.html">string</a>Program to find in the path.(path"yReturns the <code>path</code> of the corresponding program or <code>None</code> if there is no such program in the path.
O
workspace_rootpath"7The path to the root workspace of the bazel invocation.ﬂThe context of the repository rule containing helper functions and information about attributes. You get a repository_ctx object as an argument to the <code>implementation</code> function when you create a repository rule.

ô
repository_osú
archstring"ãA string identifying the architecture Bazel is running on (the value of the <code>"os.arch"</code> Java property converted to lower case).
á
environdict"ıThe dictionary of environment variables. <p><b>NOTE</b>: Retrieving an environment variable from this dictionary does not establish a dependency from a repository rule or module extension to the environment variable. To establish a dependency when looking up an environment variable, use either <code>repository_ctx.getenv</code> or <code>module_ctx.getenv</code> instead.
†
namestring"èA string identifying the operating system Bazel is running on (the value of the <code>"os.name"</code> Java property converted to lower case).
<Various data about the current platform Bazel is running on.
¸
repository_ruleËA callable value that may be invoked within the implementation function of a module extension to instantiate and return a repository rule. Created by <a href="../globals/bzl.html#repository_rule"><code>repository_rule()</code></a>.

˘
rootP
pathstring"@Returns the relative path from the exec root to the actual root.ûA root for files. The roots are the directories containing files, and they are mapped together into a single directory tree to form the execution environment.
É
rule˙A callable value representing the type of a native or Starlark rule (created by
<a href="../globals/bzl.html#rule"><code>rule()</code></a>). Calling the value during
evaluation of a package's BUILD file creates an instance of the rule and adds it to the
package's target set. For more information, visit this page about
<a href ="https://bazel.build/extending/rules">Rules</a>.

õ
rule_attributes±
attrstruct"†A struct to access the values of the <a href='https://bazel.build/extending/rules#attributes'>attributes</a>. The values are provided by the user (if not, a default value is used). The attributes of the struct and the types of their values correspond to the keys and values of the <a href='../globals/bzl.html#rule.attrs'><code>attrs</code> dict</a> provided to the <a href='../globals/bzl.html#rule'><code>rule</code> function</a>. <a href="https://github.com/bazelbuild/examples/blob/main/rules/attributes/printer.bzl">See example of use</a>.ë
exec_groupsExecGroupCollection"mA collection of the execution groups available for the rule the aspect is applied to, indexed by their names.‚

executablestruct"ÀA <code>struct</code> containing executable files defined in <a href='../toplevel/attr.html#label'>label type attributes</a> marked as <a href='../toplevel/attr.html#label.executable'><code>executable=True</code></a>. The struct fields correspond to the attribute names. Each value in the struct is either a <a href='../builtins/File.html'><code>File</code></a> or <code>None</code>. If an optional attribute is not specified in the rule then the corresponding struct value is <code>None</code>. If a label type is not marked as <code>executable=True</code>, no corresponding struct field is generated. <a href="https://github.com/bazelbuild/examples/blob/main/rules/actions_run/execute.bzl">See example of use</a>.Ÿ
filestruct"»A <code>struct</code> containing files defined in <a href='../toplevel/attr.html#label'>label type attributes</a> marked as <a href='../toplevel/attr.html#label.allow_single_file'><code>allow_single_file</code></a>. The struct fields correspond to the attribute names. The struct value is always a <a href='../builtins/File.html'><code>File</code></a> or <code>None</code>. If an optional attribute is not specified in the rule then the corresponding struct value is <code>None</code>. If a label type is not marked as <code>allow_single_file</code>, no corresponding struct field is generated. It is a shortcut for:<pre class=language-python>list(ctx.attr.&lt;ATTR&gt;.files)[0]</pre>In other words, use <code>file</code> to access the (singular) <a href="https://bazel.build/extending/rules#requesting_output_files">default output</a> of a dependency. <a href="https://github.com/bazelbuild/examples/blob/main/rules/expand_template/hello.bzl">See example of use</a>.Œ
filesstruct"ºA <code>struct</code> containing files defined in <a href='../toplevel/attr.html#label'>label</a> or <a href='../toplevel/attr.html#label_list'>label list</a> type attributes. The struct fields correspond to the attribute names. The struct values are <code>list</code> of <a href='../builtins/File.html'><code>File</code></a>s.  It is a shortcut for:<pre class=language-python>[f for t in ctx.attr.&lt;ATTR&gt; for f in t.files]</pre> In other words, use <code>files</code> to access the <a href="https://bazel.build/extending/rules#requesting_output_files"> default outputs</a> of a dependency. <a href="https://github.com/bazelbuild/examples/blob/main/rules/depsets/foo.bzl">See example of use</a>.8
kindstring"(The kind of a rule, such as 'cc_library'k

toolchainsToolchainContext"KToolchains for the default exec group of the rule the aspect is applied to.F
vardict"9Dictionary (String to String) of configuration variables.?Information about attributes of a rule an aspect is applied to.
∑
runfilesB
empty_filenamesdepset"'Returns names of empty files to create.6
filesdepset"%Returns the set of runfiles as files.‚
merge
s
other?<a class="anchor" href="../builtins/runfiles.html">runfiles</a>'The runfiles object to merge into this.(runfiles"◊Returns a new runfiles object that includes all the contents of this one and the argument. <p>
<i>Note:</i> When you have many runfiles objects to merge, use <a href="#merge_all"><code>merge_all()</code></a> rather than calling <code>merge</code> in a loop. This avoids constructing deep depset structures which can cause build failures. </p>
œ
	merge_all…
º
other{<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/runfiles.html">runfiles</a>s4The sequence of runfiles objects to merge into this.(runfiles"vReturns a new runfiles object that includes all the contents of this one and of the runfiles objects in the argument.
:
root_symlinksdepset"!Returns the set of root symlinks.0
symlinksdepset"Returns the set of symlinks.âA container of information regarding a set of files required at runtime by an executable. This object should be passed via <a href="../providers/DefaultInfo.html"><code>DefaultInfo</code></a> in order to tell the build system about the runfiles needed by the outputs produced by the rule.
<p>
    See <a href="https://bazel.build/extending/rules#runfiles">runfiles guide</a> for details. </p>

 
structøA generic object with fields.<p>Structs fields cannot be reassigned once the struct is created. Two structs are equal if they have the same fields and if corresponding field values are equal.
’
Subrule…Experimental: a building block for writing rules with shared code. For more information, please see the subrule proposal: https://docs.google.com/document/d/1RbNC88QieKvBEwir7iV5zZU08AaMlOzxhVkPnmKDedQ
¬
subrule_ctxa
actionsactions"MContains methods for declaring output files and the actions that produce themY
	fragments	fragments"AAllows access to configuration fragments in target configuration.@
labelLabel"0The label of the target currently being analyzedm

toolchainsToolchainContext"MContains methods for declaring output files and the actions that produce themDA context object passed to the implementation function of a subrule.
ø
SymlinkEntry<
pathstring",The path of the symlink in the runfiles tree/
target_fileFile"Target file of the symlink@A single runfiles symlink represented by a link name and target.
à
	tag_class{Defines a schema of attributes for a tag, created by <a href="../globals/bzl.html#tag_class"><code>tag_class()</code></a>.

ó
TargetåThe BUILD target for a dependency. Appears in the fields of <code><a href='../builtins/ctx.html#attr'>ctx.attr</a></code> corresponding to <a href='https://bazel.build/extending/rules#dependency_attributes'>dependency attributes</a> (<code><a href='../toplevel/attr.html#label'>label</a></code> or <code><a href='../toplevel/attr.html#label_list'>label_list</a></code>). Has the following fields:
<ul>
<li><h3 id='modules.Target.label'>label</h3>
<code><a href='../builtins/Label.html'>Label</a> Target.label</code><br/>
The identifier of the target.</li>
<li><h3 id='modules.Target.providers'>Providers</h3>
The <a href='https://bazel.build/extending/rules#providers'>providers</a> of a rule target can be accessed by type using index notation (<code>target[DefaultInfo]</code>). The presence of providers can be checked using the <code>in</code> operator (<code>SomeInfo in target</code>).<br/>
<br/>
</ul>
Õ
template_ctx`
argsArgs"PReturns an Args object that can be used to build memory-efficient command lines.◊
declare_file˛
z
filename7<a class="anchor" href="../core/string.html">string</a>3The relative path of the file within the directory.(
z
	directory7<a class="anchor" href="../builtins/File.html">File</a>2The directory in which the file should be created.(File"≈Declares that implementation creates a file with the given filename within the specified directory.<p>Remember that in addition to declaring a file, you must separately create an action that emits the file. Creating that action will require passing the returned <code>File</code> object to the action's construction function.¥
runÄ
©
outputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s'List of the output files of the action.(
Ù
inputs≥<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <a class="anchor" href="../builtins/depset.html">depset</a>0List or depset of the input files of the action."[]
é

executableÃ<a class="anchor" href="../builtins/File.html">File</a>; or <a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../providers/FilesToRunProvider.html">FilesToRunProvider</a>/The executable file to be called by the action.(
’
toolsç<a class="anchor" href="../core/list.html">sequence</a>; or <a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>µList or <a href="../builtins/depset.html"><code>depset</code></a> of any tools needed by the action. Tools are executable inputs that may have their own runfiles which are automatically made available to the action. <p>
When a list is provided, it can be a heterogenous collection of:
<ul>
    <li><code>File</code>s</li>
    <li><code>FilesToRunProvider</code> instances</li>
    <li><code>depset</code>s of <code>File</code>s</li>
</ul>
<code>File</code>s from <a href="../builtins/ctx#executable"><code>ctx.executable</code></a> and <code>FilesToRunProvider</code>s which are directly in the list will have their runfiles automatically added. All tools are implicitly added as inputs.
</p>
"None
≈
	arguments7<a class="anchor" href="../core/list.html">sequence</a>{Command line arguments of the action. Must be a list of strings or <a href="#args"><code>actions.args()</code></a> objects."[]
ü
progress_messageM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>6Progress message to show to the user during the build."NoneNoneType"*Creates an action that runs an executable.JA context object that is passed to the action template expansion function.
Œ
TemplateDictŒ
add≤
N
key7<a class="anchor" href="../core/string.html">string</a>A String key(
R
value7<a class="anchor" href="../core/string.html">string</a>A String value(TemplateDict"Add a String valueﬂ

add_joined∫
N
key7<a class="anchor" href="../core/string.html">string</a>A String key(
o
values;<a class="anchor" href="../builtins/depset.html">depset</a>&The depset whose items will be joined.(
Ñ
	join_with7<a class="anchor" href="../core/string.html">string</a>ªA delimiter string used to join together the strings obtained from applying <code>map_each</code>, in the same manner as <a href='../core/string.html#join'><code>string.join()</code></a>.(
Û
map_eachcallable⁄A Starlark function accepting a single argument and returning either a string, <code>None</code>, or a list of strings. This function is applied to each item of the depset specified in the <code>values</code> parameter(
Ô
uniquify3<a class="anchor" href="../core/bool.html">bool</a>¶If true, duplicate strings derived from <code>values</code> will be omitted. Only the first occurrence of each string will remain. Usually this feature is not needed because depsets already omit duplicates, but it can be useful if <code>map_each</code> emits the same string for multiple items."False
ﬂ
format_joinedM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>yAn optional format string pattern applied to the joined string. The format string must have exactly one '%s' placeholder."None
ó
allow_closure3<a class="anchor" href="../core/bool.html">bool</a>…If true, allows the use of closures in function parameters like <code>map_each</code>. Usually this isn't necessary and it risks retaining large analysis-phase data structures into the execution phase."FalseTemplateDict"Add depset of valuesäAn Args-like structure for use in ctx.actions.expand_template(), which allows for deferring evaluation of values till the execution phase.
€
toolchain_typeG
	mandatorybool"4Whether the toolchain type is mandatory or optional.=
toolchain_typeLabel"$The toolchain type that is required.AA data type describing a dependency on a specific toolchain type.
Ë
ToolchainContext”Holds toolchains available for a particular exec group. Toolchain targets are accessed by indexing with the toolchain type, as in <code>ctx.toolchains["//pkg:my_toolchain_type"]</code>. If the toolchain was optional and no toolchain was resolved, this will return <code>None</code>. Accessing toolchains of an aspect or rule via <code>ctx.toolchains</code> returns the indexed toolchain as a <code>ToolchainInfo</code> provider. While when using aspects, <code>ToolchainContext</code> is also used to hold the toolchains of the base target. It can be accessed by <code>ctx.rule.toolchains["//pkg:my_toolchain_type"]</code> and it returns the list of providers resulted from applying the aspects on these toolchain targets. 
˘

transitionÍ<p>Represents a configuration transition across a dependency edge. For example, if <code>//package:foo</code> depends on <code>//package:bar</code> with a configuration transition, then the configuration of <code>//package:bar</code> (and its dependencies) will be <code>//package:foo</code>'s configuration plus the changes specified by the transition function.
î
wasm_exec_resultd
error_messagestring"KContains an error message if execution failed before the function returned.X
outputstring"FThe content of the output buffer returned by the WebAssembly function.ê
return_codeint"|The return value of the WebAssembly function, or a negative value if execution
was terminated before the function returned.
¨The result of executing a WebAssembly function with
<code>repository_ctx.execute_wasm()</code>. It contains the function's
return value and output buffer.

<p>If execution failed before the function returned then the return code will be negative
and the <code>error_message</code> field will be set.

ö
wasm_moduleB
pathunknown"1The path this WebAssembly module was loaded from.GA WebAssembly module loaded by <code>repository_ctx.load_wasm()</code>.
†
apple_common÷
XcodePropertiesProvider"∏The constructor/key for the <code>XcodeVersionProperties</code> provider.<p>If a target propagates the <code>XcodeVersionProperties</code> provider, use this as the key with which to retrieve it. Example:<br><pre class='language-python'>
dep = ctx.attr.deps[0]
p = dep[apple_common.XcodeVersionProperties]
</pre>e
XcodeVersionConfigProvider"EThe constructor/key for the <code>XcodeVersionConfig</code> provider.ë
apple_host_system_envZ
R
xcode_config@A provider containing information about the Xcode configuration.(dict"õReturns a <a href='../core/dict.html'>dict</a> of environment variables that should be set for actions that need to run build tools on an Apple host system, such as the  version of Xcode that should be used. The keys are variable names and the values  are their corresponding values.U
apple_toolchain	unknown"7Utilities for resolving items from the apple toolchain.Ì
dotted_versionÜ
u
version7<a class="anchor" href="../core/string.html">string</a>/The string representation of the DottedVersion.(DottedVersion"RCreates a new <a href="../builtins/DottedVersion.html">DottedVersion</a> instance.Û
platformstruct"ﬁAn enum-like struct that contains the following fields corresponding to Apple platforms:<br><ul><li><code>ios_device</code></li><li><code>ios_simulator</code></li><li><code>macos</code></li><li><code>tvos_device</code></li><li><code>tvos_simulator</code></li><li><code>visionos_device</code></li><li><code>visionos_simulator</code></li><li><code>watchos_device</code></li><li><code>watchos_simulator</code></li></ul><p>These values can be passed to methods that expect a platform, like <a href='../providers/XcodeVersionConfig.html#sdk_version_for_platform'>XcodeVersionConfig.sdk_version_for_platform</a>.“
platform_typestruct"∏An enum-like struct that contains the following fields corresponding to Apple platform types:<br><ul><li><code>ios</code></li><li><code>macos</code></li><li><code>tvos</code></li><li><code>visionos</code></li><li><code>watchos</code></li></ul><p>These values can be passed to methods that expect a platform type.⁄
target_apple_env}
R
xcode_config@A provider containing information about the Xcode configuration.(
!
platformThe apple platform.(dict"∆Returns a <code>dict</code> of environment variables that should be set for actions that build targets of the given Apple platform type. For example, this dictionary contains variables that denote the platform name and SDK version with which to build. The keys are variable names and the values are their corresponding values.MFunctions for Starlark to access internals of the apple rule implementations.
Úﬂ
attró
bool≈

∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
¢
default3<a class="anchor" href="../core/bool.html">bool</a>[A default value to use if no value for this attribute is given when instantiating the rule."False
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False	Attribute"∆Creates a schema for a boolean attribute. The corresponding <a href='../builtins/ctx.html#attr'><code>ctx.attr</code></a> attribute will be of type <a href='../core/bool.html'><code>bool</code></a>.ö
intú
∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
ú
default1<a class="anchor" href="../core/int.html">int</a>[A default value to use if no value for this attribute is given when instantiating the rule."0
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
⁄
valuesm<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/int.html">int</a>s]The list of allowed values for the attribute. An error is raised if any other value is given."[]	Attribute"ÛCreates a schema for an integer attribute. The value must be in the signed 32-bit range. The corresponding <a href='../builtins/ctx.html#attr'><code>ctx.attr</code></a> attribute will be of type <a href='../core/int.html'><code>int</code></a>.›
int_listÎ
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
m
allow_empty3<a class="anchor" href="../core/bool.html">bool</a>#True if the attribute can be empty."True
∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
Ÿ
defaultm<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/int.html">int</a>s[A default value to use if no value for this attribute is given when instantiating the rule."[]
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None	Attribute"cCreates a schema for a list-of-integers attribute. Each element must be in the signed 32-bit range.£<
labelˆ0
∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
ÿ
defaultπ<a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/LateBoundDefault.html">LateBoundDefault</a>; or NativeComputedDefault; or <a class="anchor" href="../core/function.html">function</a>; or <code>None</code>äA default value to use if no value for this attribute is given when instantiating the rule.Use a string or the <a href="../builtins/Label.html#Label"><code>Label</code></a> function to specify a default value, for example, <code>attr.label(default = "//a:b")</code>."None
ù
materializer;<a class="anchor" href="../core/function.html">function</a>…<b>Experimental</b>. This parameter is experimental and may change at any time. Please do not depend on it. It may be enabled on an experimental basis by setting <code>--experimental_dormant_deps</code> <br>If set, the attribute materializes dormant dependencies from the transitive closure. The value of this parameter must be a functon that gets access to the values of the attributes of the rule that either are not dependencies or are marked as available for dependency resolution. It must return either a dormant dependency or a list of them depending on the type of the attribute"None
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
¶

executable3<a class="anchor" href="../core/bool.html">bool</a>€True if the dependency has to be executable. This means the label must refer to an executable file, or to a rule that outputs an executable file. Access the label with <code>ctx.executable.&lt;attribute_name&gt;</code>."False
õ
allow_files¡<a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>¡Whether <code>File</code> targets are allowed. Can be <code>True</code>, <code>False</code> (default), or a list of file extensions that are allowed (for example, <code>[".cc", ".cpp"]</code>)."None
Ú
allow_single_file÷This is similar to <code>allow_files</code>, with the restriction that the label must correspond to a single <a href="../builtins/File.html">File</a>. Access it through <code>ctx.file.&lt;attribute_name&gt;</code>."None
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
Î
skip_validations3<a class="anchor" href="../core/bool.html">bool</a>öIf true, validation actions of transitive dependencies from this attribute will not run. This is a temporary mitigation and WILL be removed in the future."False
∑
	providers7<a class="anchor" href="../core/list.html">sequence</a>ÏThe providers that must be given by any dependency appearing in this attribute.<p>The format of this argument is a list of lists of providers -- <code>*Info</code> objects returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a> (or in the case of a legacy provider, its string name). The dependency must return ALL providers mentioned in at least ONE of the inner lists. As a convenience, this argument may also be a single-level list of providers, in which case it is wrapped in an outer list with one element (i.e. <code>[A, B]</code> means <code>[[A, B]]</code>). It is NOT required that the rule of the dependency advertises those providers in its <code>provides</code> parameter, however, it is considered best practice."[]
À
for_dependency_resolution§If this is set, the attribute is available for materializers. Only rules marked with the flag of the same name are allowed to be referenced through such attributes."unbound
ü
allow_rulesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>~Which rule targets (name of the classes) are allowed. This is deprecated (kept only for compatibility), use providers instead."None
˚
cfgÌ<a href="https://bazel.build/extending/rules#configurations">Configuration</a> of the attribute. It can be either <code>"exec"</code>, which indicates that the dependency is built for the <code>execution platform</code>, or <code>"target"</code>, which indicates that the dependency is build for the <code>target platform</code>. A typical example of the difference is when building mobile apps, where the <code>target platform</code> is <code>Android</code> or <code>iOS</code> while the <code>execution platform</code> is <code>Linux</code>, <code>macOS</code>, or <code>Windows</code>. This parameter is required if <code>executable</code> is True to guard against accidentally building host tools in the target configuration. <code>"target"</code> has no semantic effect, so don't set it when <code>executable</code> is False unless it really helps clarify your intentions."None
Â
aspectsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Aspect.html">Aspect</a>s]Aspects that should be applied to the dependency or dependencies specified by this attribute."[]
û
flagss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sDeprecated, will be removed."[]	Attribute"†<p>Creates a schema for a label attribute. This is a dependency attribute.</p><p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time (within the rule's implementation function), when retrieving the attribute value from <code>ctx.attr</code>, labels are replaced by the corresponding <a href='../builtins/Target.html'><code>Target</code></a>s. This allows you to access the providers of the current target's dependencies.<p>In addition to ordinary source files, this kind of attribute is often used to refer to a tool -- for example, a compiler. Such tools are considered to be dependencies, just like source files. To avoid requiring users to specify the tool's label every time they use the rule in their BUILD files, you can hard-code the label of a canonical tool as the <code>default</code> value of this attribute. If you also want to prevent users from overriding this default, you can make the attribute private by giving it a name that starts with an underscore. See the <a href='https://bazel.build/extending/rules#private-attributes'>Rules</a> page for more information.‹+
label_keyed_string_dictÎ$
m
allow_empty3<a class="anchor" href="../core/bool.html">bool</a>#True if the attribute can be empty."True
∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
Ω
defaults<a class="anchor" href="../core/dict.html">dict</a>; or <a class="anchor" href="../core/function.html">function</a>∏A default value to use if no value for this attribute is given when instantiating the rule.Use strings or the <a href="../builtins/Label.html#Label"><code>Label</code></a> function to specify default values, for example, <code>attr.label_keyed_string_dict(default = {"//a:b": "value", "//a:c": "string"})</code>."{}
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
õ
allow_files¡<a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>¡Whether <code>File</code> targets are allowed. Can be <code>True</code>, <code>False</code> (default), or a list of file extensions that are allowed (for example, <code>[".cc", ".cpp"]</code>)."None
ü
allow_rulesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>~Which rule targets (name of the classes) are allowed. This is deprecated (kept only for compatibility), use providers instead."None
∑
	providers7<a class="anchor" href="../core/list.html">sequence</a>ÏThe providers that must be given by any dependency appearing in this attribute.<p>The format of this argument is a list of lists of providers -- <code>*Info</code> objects returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a> (or in the case of a legacy provider, its string name). The dependency must return ALL providers mentioned in at least ONE of the inner lists. As a convenience, this argument may also be a single-level list of providers, in which case it is wrapped in an outer list with one element (i.e. <code>[A, B]</code> means <code>[[A, B]]</code>). It is NOT required that the rule of the dependency advertises those providers in its <code>provides</code> parameter, however, it is considered best practice."[]
À
for_dependency_resolution§If this is set, the attribute is available for materializers. Only rules marked with the flag of the same name are allowed to be referenced through such attributes."unbound
û
flagss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sDeprecated, will be removed."[]
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
Î
skip_validations3<a class="anchor" href="../core/bool.html">bool</a>öIf true, validation actions of transitive dependencies from this attribute will not run. This is a temporary mitigation and WILL be removed in the future."False
⁄
cfgÃ<a href="https://bazel.build/extending/rules#configurations">Configuration</a> of the attribute. It can be either <code>"exec"</code>, which indicates that the dependency is built for the <code>execution platform</code>, or <code>"target"</code>, which indicates that the dependency is build for the <code>target platform</code>. A typical example of the difference is when building mobile apps, where the <code>target platform</code> is <code>Android</code> or <code>iOS</code> while the <code>execution platform</code> is <code>Linux</code>, <code>macOS</code>, or <code>Windows</code>."None
Â
aspectsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Aspect.html">Aspect</a>s]Aspects that should be applied to the dependency or dependencies specified by this attribute."[]	Attribute"“<p>Creates a schema for an attribute holding a dictionary, where the keys are labels and the values are strings. This is a dependency attribute.</p><p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time (within the rule's implementation function), when retrieving the attribute value from <code>ctx.attr</code>, labels are replaced by the corresponding <a href='../builtins/Target.html'><code>Target</code></a>s. This allows you to access the providers of the current target's dependencies.£2

label_listÆ*
m
allow_empty3<a class="anchor" href="../core/bool.html">bool</a>#True if the attribute can be empty."True
∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
‡
defaultµ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Label.html">Label</a>s; or <a class="anchor" href="../core/function.html">function</a>òA default value to use if no value for this attribute is given when instantiating the rule.Use strings or the <a href="../builtins/Label.html#Label"><code>Label</code></a> function to specify default values, for example, <code>attr.label_list(default = ["//a:b", "//a:c"])</code>."[]
ù
materializer;<a class="anchor" href="../core/function.html">function</a>…<b>Experimental</b>. This parameter is experimental and may change at any time. Please do not depend on it. It may be enabled on an experimental basis by setting <code>--experimental_dormant_deps</code> <br>If set, the attribute materializes dormant dependencies from the transitive closure. The value of this parameter must be a functon that gets access to the values of the attributes of the rule that either are not dependencies or are marked as available for dependency resolution. It must return either a dormant dependency or a list of them depending on the type of the attribute"None
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
õ
allow_files¡<a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>¡Whether <code>File</code> targets are allowed. Can be <code>True</code>, <code>False</code> (default), or a list of file extensions that are allowed (for example, <code>[".cc", ".cpp"]</code>)."None
ü
allow_rulesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>~Which rule targets (name of the classes) are allowed. This is deprecated (kept only for compatibility), use providers instead."None
∑
	providers7<a class="anchor" href="../core/list.html">sequence</a>ÏThe providers that must be given by any dependency appearing in this attribute.<p>The format of this argument is a list of lists of providers -- <code>*Info</code> objects returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a> (or in the case of a legacy provider, its string name). The dependency must return ALL providers mentioned in at least ONE of the inner lists. As a convenience, this argument may also be a single-level list of providers, in which case it is wrapped in an outer list with one element (i.e. <code>[A, B]</code> means <code>[[A, B]]</code>). It is NOT required that the rule of the dependency advertises those providers in its <code>provides</code> parameter, however, it is considered best practice."[]
À
for_dependency_resolution§If this is set, the attribute is available for materializers. Only rules marked with the flag of the same name are allowed to be referenced through such attributes."unbound
û
flagss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sDeprecated, will be removed."[]
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
Î
skip_validations3<a class="anchor" href="../core/bool.html">bool</a>öIf true, validation actions of transitive dependencies from this attribute will not run. This is a temporary mitigation and WILL be removed in the future."False
⁄
cfgÃ<a href="https://bazel.build/extending/rules#configurations">Configuration</a> of the attribute. It can be either <code>"exec"</code>, which indicates that the dependency is built for the <code>execution platform</code>, or <code>"target"</code>, which indicates that the dependency is build for the <code>target platform</code>. A typical example of the difference is when building mobile apps, where the <code>target platform</code> is <code>Android</code> or <code>iOS</code> while the <code>execution platform</code> is <code>Linux</code>, <code>macOS</code>, or <code>Windows</code>."None
Â
aspectsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Aspect.html">Aspect</a>s]Aspects that should be applied to the dependency or dependencies specified by this attribute."[]	Attribute"„<p>Creates a schema for a list-of-labels attribute. This is a dependency attribute. The corresponding <a href='../builtins/ctx.html#attr'><code>ctx.attr</code></a> attribute will be of type <a href='../core/list.html'>list</a> of <a href='../builtins/Target.html'><code>Target</code>s</a>.</p><p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time (within the rule's implementation function), when retrieving the attribute value from <code>ctx.attr</code>, labels are replaced by the corresponding <a href='../builtins/Target.html'><code>Target</code></a>s. This allows you to access the providers of the current target's dependencies.Æ+
label_list_dictΩ$
m
allow_empty3<a class="anchor" href="../core/bool.html">bool</a>#True if the attribute can be empty."True
∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
è
default3<a class="anchor" href="../core/dict.html">dict</a> A default value to use if no value for this attribute is given when instantiating the rule.Use strings or the <a href="../builtins/Label.html#Label"><code>Label</code>
</a> function to specify default values, for example,
<code>attr.label_list_dict(default = {"key1": ["//a:b", "//a:c"], "key2":
[Label("@my_repo//d:e")]})</code>."{}
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
õ
allow_files¡<a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>¡Whether <code>File</code> targets are allowed. Can be <code>True</code>, <code>False</code> (default), or a list of file extensions that are allowed (for example, <code>[".cc", ".cpp"]</code>)."None
ü
allow_rulesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>~Which rule targets (name of the classes) are allowed. This is deprecated (kept only for compatibility), use providers instead."None
∑
	providers7<a class="anchor" href="../core/list.html">sequence</a>ÏThe providers that must be given by any dependency appearing in this attribute.<p>The format of this argument is a list of lists of providers -- <code>*Info</code> objects returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a> (or in the case of a legacy provider, its string name). The dependency must return ALL providers mentioned in at least ONE of the inner lists. As a convenience, this argument may also be a single-level list of providers, in which case it is wrapped in an outer list with one element (i.e. <code>[A, B]</code> means <code>[[A, B]]</code>). It is NOT required that the rule of the dependency advertises those providers in its <code>provides</code> parameter, however, it is considered best practice."[]
À
for_dependency_resolution§If this is set, the attribute is available for materializers. Only rules marked with the flag of the same name are allowed to be referenced through such attributes."unbound
û
flagss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sDeprecated, will be removed."[]
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
Î
skip_validations3<a class="anchor" href="../core/bool.html">bool</a>öIf true, validation actions of transitive dependencies from this attribute will not run. This is a temporary mitigation and WILL be removed in the future."False
⁄
cfgÃ<a href="https://bazel.build/extending/rules#configurations">Configuration</a> of the attribute. It can be either <code>"exec"</code>, which indicates that the dependency is built for the <code>execution platform</code>, or <code>"target"</code>, which indicates that the dependency is build for the <code>target platform</code>. A typical example of the difference is when building mobile apps, where the <code>target platform</code> is <code>Android</code> or <code>iOS</code> while the <code>execution platform</code> is <code>Linux</code>, <code>macOS</code>, or <code>Windows</code>."None
Â
aspectsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Aspect.html">Aspect</a>s]Aspects that should be applied to the dependency or dependencies specified by this attribute."[]	Attribute"⁄<p>Creates a schema for an attribute holding a dictionary, where the keys are strings and the values are list of labels. This is a dependency attribute.</p><p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time (within the rule's implementation function), when retrieving the attribute value from <code>ctx.attr</code>, labels are replaced by the corresponding <a href='../builtins/Target.html'><code>Target</code></a>s. This allows you to access the providers of the current target's dependencies.Ó
outputÂ
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False	Attribute"˚<p>Creates a schema for an output (label) attribute.</p><p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time, the corresponding <a href='../builtins/File.html'><code>File</code></a> can be retrieved using <a href='../builtins/ctx.html#outputs'><code>ctx.outputs</code></a>.€
output_list‘
m
allow_empty3<a class="anchor" href="../core/bool.html">bool</a>#True if the attribute can be empty."True
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False	Attribute"ÙCreates a schema for a list-of-outputs attribute.<p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time, the corresponding <a href='../builtins/File.html'><code>File</code></a> can be retrieved using <a href='../builtins/ctx.html#outputs'><code>ctx.outputs</code></a>.ü
string√
∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
Ω
defaultQ<a class="anchor" href="../core/string.html">string</a>; or NativeComputedDefault[A default value to use if no value for this attribute is given when instantiating the rule."''
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
‡
valuess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s]The list of allowed values for the attribute. An error is raised if any other value is given."[]	Attribute"OCreates a schema for a <a href='../core/string.html#attr'>string</a> attribute.°
string_dict±
m
allow_empty3<a class="anchor" href="../core/bool.html">bool</a>#True if the attribute can be empty."True
∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
ü
default3<a class="anchor" href="../core/dict.html">dict</a>[A default value to use if no value for this attribute is given when instantiating the rule."{}
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False	Attribute"^Creates a schema for an attribute holding a dictionary, where the keys and values are strings.Ô)
string_keyed_label_dict¯"
m
allow_empty3<a class="anchor" href="../core/bool.html">bool</a>#True if the attribute can be empty."True
∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
∏
defaults<a class="anchor" href="../core/dict.html">dict</a>; or <a class="anchor" href="../core/function.html">function</a>≥A default value to use if no value for this attribute is given when instantiating the rule.Use strings or the <a href="../builtins/Label.html#Label"><code>Label</code></a> function to specify default values, for example, <code>attr.string_keyed_label_dict(default = {"foo": "//a:b", "bar": "//a:c"})</code>."{}
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
õ
allow_files¡<a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>¡Whether <code>File</code> targets are allowed. Can be <code>True</code>, <code>False</code> (default), or a list of file extensions that are allowed (for example, <code>[".cc", ".cpp"]</code>)."None
ü
allow_rulesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>~Which rule targets (name of the classes) are allowed. This is deprecated (kept only for compatibility), use providers instead."None
∑
	providers7<a class="anchor" href="../core/list.html">sequence</a>ÏThe providers that must be given by any dependency appearing in this attribute.<p>The format of this argument is a list of lists of providers -- <code>*Info</code> objects returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a> (or in the case of a legacy provider, its string name). The dependency must return ALL providers mentioned in at least ONE of the inner lists. As a convenience, this argument may also be a single-level list of providers, in which case it is wrapped in an outer list with one element (i.e. <code>[A, B]</code> means <code>[[A, B]]</code>). It is NOT required that the rule of the dependency advertises those providers in its <code>provides</code> parameter, however, it is considered best practice."[]
À
for_dependency_resolution§If this is set, the attribute is available for materializers. Only rules marked with the flag of the same name are allowed to be referenced through such attributes."unbound
û
flagss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sDeprecated, will be removed."[]
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
⁄
cfgÃ<a href="https://bazel.build/extending/rules#configurations">Configuration</a> of the attribute. It can be either <code>"exec"</code>, which indicates that the dependency is built for the <code>execution platform</code>, or <code>"target"</code>, which indicates that the dependency is build for the <code>target platform</code>. A typical example of the difference is when building mobile apps, where the <code>target platform</code> is <code>Android</code> or <code>iOS</code> while the <code>execution platform</code> is <code>Linux</code>, <code>macOS</code>, or <code>Windows</code>."None
Â
aspectsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Aspect.html">Aspect</a>s]Aspects that should be applied to the dependency or dependencies specified by this attribute."[]	Attribute"ÿ<p>Creates a schema for an attribute whose value is a dictionary where the keys are strings and the values are labels. This is a dependency attribute.</p><p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time (within the rule's implementation function), when retrieving the attribute value from <code>ctx.attr</code>, labels are replaced by the corresponding <a href='../builtins/Target.html'><code>Target</code></a>s. This allows you to access the providers of the current target's dependencies.œ
string_listå
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
m
allow_empty3<a class="anchor" href="../core/bool.html">bool</a>#True if the attribute can be empty."True
∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
˙
defaultç<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or NativeComputedDefault[A default value to use if no value for this attribute is given when instantiating the rule."[]
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None	Attribute"1Creates a schema for a list-of-strings attribute.ø
string_list_dict±
m
allow_empty3<a class="anchor" href="../core/bool.html">bool</a>#True if the attribute can be empty."True
∏
configurable?<a class="anchor" href="../core/bool.html">bool</a>; or unbound›This argument can only be specified for an attribute of a symbolic macro.<p>If <code>configurable</code> is explicitly set to <code>False</code>, the symbolic macro attribute is non-configurable - in other words, it cannot take a <code>select()</code> value. If the <code>configurable</code> is either unbound or explicitly set to <code>True</code>, the attribute is configurable and can take a <code>select()</code> value.<p>For an attribute of a rule or aspect, <code>configurable</code> must be left unbound. Most Starlark rule attributes are always configurable, with the exception of <code>attr.output()</code>, <code>attr.output_list()</code>, and <code>attr.license()</code> rule attributes, which are always non-configurable."unbound
ü
default3<a class="anchor" href="../core/dict.html">dict</a>[A default value to use if no value for this attribute is given when instantiating the rule."{}
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
°
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>XIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False	Attribute"wCreates a schema for an attribute holding a dictionary, where the keys are strings and the values are lists of strings.€This is a top-level module for defining the attribute schemas of a rule or aspect. Each function returns an object representing the schema of a single attribute. These objects are used as the values of the <code>attrs</code> dictionary argument of <a href="../globals/bzl.html#rule"><code>rule()</code></a>, <a href="../globals/bzl.html#aspect"><code>aspect()</code></a>, <a href="../globals/bzl.html#repository_rule"><code>repository_rule()</code></a> and <a href="../globals/bzl.html#tag_class"><code>tag_class()</code></a>. <p>See the Rules page for more on <a href="https://bazel.build/extending/rules#attributes">defining</a>
and <a href="https://bazel.build/extending/rules#implementation_function">using</a> attributes.</p>

˘˚
	cc_common
CcToolchainInfoProvider"bThe key used to retrieve the provider that contains information about the C++ toolchain being usedÎ
action_is_enabledá
ò
feature_configurationW<a class="anchor" href="../builtins/FeatureConfiguration.html">FeatureConfiguration</a>$Feature configuration to be queried.(
d
action_name7<a class="anchor" href="../core/string.html">string</a>Name of the action_config.(bool"LReturns True if given action_config is enabled in the feature configuration.π$
compile±#
h
actions=<a class="anchor" href="../builtins/actions.html">actions</a><code>actions</code> object.(
•
feature_configurationW<a class="anchor" href="../builtins/FeatureConfiguration.html">FeatureConfiguration</a>1<code>feature_configuration</code> to be queried.(
I
cc_toolchainInfo1<code>CcToolchainInfo</code> provider to be used.(
m
srcs7<a class="anchor" href="../core/list.html">sequence</a>(The list of source files to be compiled."[]
Ø
public_hdrs7<a class="anchor" href="../core/list.html">sequence</a>cList of headers needed for compilation of srcs and may be included by dependent rules transitively."[]
¶
private_hdrs7<a class="anchor" href="../core/list.html">sequence</a>YList of headers needed for compilation of srcs and NOT to be included by dependent rules."[]
ì
includesw<a class="anchor" href="../core/list.html">sequence</a>; or <a class="anchor" href="../builtins/depset.html">depset</a>âSearch paths for header files referenced both by angle bracket and quotes. Usually passed with -I. Propagated to dependents transitively."[]
£
quote_includes7<a class="anchor" href="../core/list.html">sequence</a>”Search paths for header files referenced by quotes, e.g. #include "foo/bar/header.h". They can be either relative to the exec root or absolute. Usually passed with -iquote. Propagated to dependents transitively."[]
≥
system_includes7<a class="anchor" href="../core/list.html">sequence</a>‚Search paths for header files referenced by angle brackets, e.g. #include &lt;foo/bar/header.h&gt;. They can be either relative to the exec root or absolute. Usually passed with -isystem. Propagated to dependents transitively."[]
Å
framework_includes7<a class="anchor" href="../core/list.html">sequence</a>≠Search paths for header files from Apple frameworks. They can be either relative to the exec root or absolute. Usually passed with -F. Propagated to dependents transitively."[]
µ
defines7<a class="anchor" href="../core/list.html">sequence</a>mSet of defines needed to compile this target. Each define is a string. Propagated to dependents transitively."[]
ø
local_defines7<a class="anchor" href="../core/list.html">sequence</a>qSet of defines needed to compile this target. Each define is a string. Not propagated to dependents transitively."[]
˚
include_prefix7<a class="anchor" href="../core/string.html">string</a>´The prefix to add to the paths of the headers of this rule. When set, the headers in the hdrs attribute of this rule are accessible at is the value of this attribute prepended to their repository-relative path. The prefix in the strip_include_prefix attribute is removed before this prefix is added."''
ﬂ
strip_include_prefix7<a class="anchor" href="../core/string.html">string</a>âThe prefix to strip from the paths of the headers of this rule. When set, the headers in the hdrs attribute of this rule are accessible at their path with this prefix cut off. If it's a relative path, it's taken as a package-relative one. If it's an absolute one, it's understood as a repository-relative path. The prefix in the include_prefix attribute is added after this prefix is stripped."''
z
user_compile_flags7<a class="anchor" href="../core/list.html">sequence</a>'Additional list of compilation options."[]
Ç
conly_flags7<a class="anchor" href="../core/list.html">sequence</a>6Additional list of compilation options for C compiles."[]
Ç
	cxx_flags7<a class="anchor" href="../core/list.html">sequence</a>8Additional list of compilation options for C++ compiles."[]
Ñ
compilation_contexts7<a class="anchor" href="../core/list.html">sequence</a>/Headers from dependencies used for compilation."[]
≤
name7<a class="anchor" href="../core/string.html">string</a>oThis is used for naming the output artifacts of actions created by this method. See also the `main_output` arg.(
z
disallow_pic_outputs3<a class="anchor" href="../core/bool.html">bool</a>&Whether PIC outputs should be created."False
~
disallow_nopic_outputs3<a class="anchor" href="../core/bool.html">bool</a>(Whether NOPIC outputs should be created."False
â
additional_inputs7<a class="anchor" href="../core/list.html">sequence</a>7List of additional files needed for compilation of srcs"[]
Ê
module_interfaces7<a class="anchor" href="../core/list.html">sequence</a>éThe list of module interfaces source files to be compiled. Note: this is an experimental feature, only enabled with --experimental_cpp_modules"unboundtuple"zShould be used for C++ compilation. Returns tuple of (<code>CompilationContext</code>, <code>CcCompilationOutputs</code>).ƒ
configure_featuresŸ
Q
ctx5<a class="anchor" href="../builtins/ctx.html">ctx</a>The rule context.(
E
cc_toolchainInfo-cc_toolchain for which we configure features.(
†
languageM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>?The language to configure for: either c++ or objc (default c++)"None
r
requested_features7<a class="anchor" href="../core/list.html">sequence</a>List of features to be enabled."[]
è
unsupported_features7<a class="anchor" href="../core/list.html">sequence</a>:List of features that are unsupported by the current rule."[]FeatureConfiguration"RCreates a feature_configuration instance. Requires the cpp configuration fragment.©<
create_cc_toolchain_config_infoŒ;
Q
ctx5<a class="anchor" href="../builtins/ctx.html">ctx</a>The rule context.(
¿
features7<a class="anchor" href="../core/list.html">sequence</a>ˆContains all flag specifications for one feature.<p>Arguments:</p><p><code>name</code>: The feature's name. It is possible to introduce a feature without a change to Bazel by adding a 'feature' section to the toolchain and adding the corresponding string as feature in the <code>BUILD</code> file.</p><p><code>enabled</code>: If 'True', this feature is enabled unless a rule type explicitly marks it as unsupported.</p><p><code>flag_sets</code>: A FlagSet list. If the given feature is enabled, the flag sets will be applied for the actions are specified for. </p><p><code>env_sets</code>: an EnvSet list. If the given feature is enabled, the env sets will be applied for the actions they are specified for. </p><p><code>requires</code>: A list of feature sets defining when this feature is supported by the  toolchain. The feature is supported if any of the feature sets fully apply, that is, when all features of a feature set are enabled. If <code>requires</code> is omitted, the feature is supported independently of which other features are enabled. Use this for example to filter flags depending on the build mode enabled (opt / fastbuild / dbg). </p><p><code>implies</code>: A string list of features or action configs that are automatically enabled when this feature is enabled. If any of the implied features or action configs cannot be enabled, this feature will (silently) not be enabled either. </p><p><code>provides</code>: A list of names this feature conflicts with. </p>A feature cannot be enabled if:</br>- <code>provides</code> contains the name of a different feature or action config that we want to enable.</br>- <code>provides</code> contains the same value as a 'provides' in a different feature or action config that we want to enable. Use this in order to ensure that incompatible features cannot be accidentally activated at the same time, leading to hard to diagnose compiler errors."[]
ú

action_configs7<a class="anchor" href="../core/list.html">sequence</a>Ã	An action config corresponds to a Bazel action, and allows selection of a tool based on activated features. Action config activation occurs by the same semantics as features: a feature can 'require' or 'imply' an action config in the same way that it would another feature.<p>Arguments:</p><p><code>action_name</code>: The name of the Bazel action that this config applies to, e.g. 'c-compile' or 'c-module-compile'.</p><p><code>enabled</code>: If 'True', this action is enabled unless a rule type explicitly marks it as unsupported.</p><p><code>tools</code>: The tool applied to the action will be the first tool with a feature set that matches the feature configuration.  An error will be thrown if no tool matches a provided feature configuration - for that reason, it's a good idea to provide a default tool with an empty feature set.</p><p><code>flag_sets</code>: If the given action config is enabled, the flag sets will be applied to the corresponding action.</p><p><code>implies</code>: A list of features or action configs that are automatically enabled when this action config is enabled. If any of the implied features or action configs cannot be enabled, this action config will (silently) not be enabled either.</p>"[]
¿
artifact_name_patterns7<a class="anchor" href="../core/list.html">sequence</a>ËThe name for an artifact of a given category of input or output artifacts to an action.<p>Arguments:</p><p><code>category_name</code>: The category of artifacts that this selection applies to. This field is compared against a list of categories defined in Bazel. Example categories include "linked_output" or the artifact for this selection. Together with the extension it is used to create an artifact name based on the target name.</p><p><code>extension</code>: The extension for creating the artifact for this selection. Together with the prefix it is used to create an artifact name based on the target name.</p>"[]
ë
cxx_builtin_include_directories7<a class="anchor" href="../core/list.html">sequence</a>∞<p>Built-in include directories for C++ compilation. These should be the exact paths used by the compiler, and are generally relative to the exec root.</p><p>The paths used by the compiler can be determined by 'gcc -E -xc++ - -v'.</p><p>We currently use the C++ paths also for C compilation, which is safe as long as there are no name clashes between C++ and C header files.</p><p>Relative paths are resolved relative to the configuration file directory.</p><p>If the compiler has --sysroot support, then these paths should use %sysroot% rather than the include path, and specify the sysroot attribute in order to give blaze the information necessary to make the correct replacements.</p>"[]
ù
toolchain_identifier7<a class="anchor" href="../core/string.html">string</a>…<p>The unique identifier of the toolchain within the crosstool release. It must be possible to use this as a directory name in a path.</p><p>It has to match the following regex: [a-zA-Z_][\.\- \w]*</p>(
q
host_system_nameM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>Ignored."None
Ã
target_system_nameM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>aDeprecated. The GNU System Name. The string is exposed to CcToolchainInfo.target_gnu_system_name."None
Ò

target_cpuM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>çDeprecated: Use cpu based constraints instead. If the string is "k8", `target_cpu` will be omitted from the filename of raw FDO profile data."None
Ω
target_libcM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>ÿDeprecated: Use OS based constraints instead. The libc version string (e.g. "glibc-2.2.2"). If the string is "macosx", platform is assumed to be MacOS. Otherwise, Linux. The string is exposed to CcToolchainInfo.libc."None
 
compiler7<a class="anchor" href="../core/string.html">string</a>ÇThe compiler string (e.g. "gcc"). The current toolchain's compiler is exposed to `@bazel_tools//tools/cpp:compiler (compiler_flag)` as a flag value. Targets that require compiler-specific flags can use the config_settings in https://github.com/bazelbuild/rules_cc/blob/main/cc/compiler/BUILD in select() statements or create custom config_setting if the existing settings don't suffice.(
Õ
abi_versionM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>iThe abi in use, which is a gcc version. E.g.: "gcc-3.4". The string is set to C++ toolchain variable ABI."None
’
abi_libc_versionM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>lThe glibc version used by the abi we're using. The string is set to C++ toolchain variable ABI_LIBC_VERSION."None
∞

tool_paths7<a class="anchor" href="../core/list.html">sequence</a>‰Tool locations.<p>Arguments:</p><p><code>name</code>: Name of the tool.</p><p><code>path</code>: Location of the tool; Can be absolute path (in case of non hermetic toolchain), or path relative to the cc_toolchain's package.</p>"[]
Ä
make_variables7<a class="anchor" href="../core/list.html">sequence</a>1A make variable that is made accessible to rules."[]
Ù
builtin_sysrootM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>ãThe built-in sysroot. If this attribute is not present, Bazel does not allow using a different sysroot, i.e. through the --grte_top option."NoneNoneType"5Creates a <code>CcToolchainConfigInfo</code> provider»
create_compilation_context˝
@
headers,Set of headers needed to compile this target"unbound
ﬁ
system_includes¡Set of search paths for header files referenced by angle brackets, i.e. #include &lt;foo/bar/header.h&gt;. They can be either relative to the exec root or absolute. Usually passed with -isystem"unbound
|
includesgSet of search paths for header files referenced both by angle bracket and quotes.Usually passed with -I"unbound
Œ
quote_includes≤Set of search paths for header files referenced by quotes, i.e. #include "foo/bar/header.h". They can be either relative to the exec root or absolute. Usually passed with -iquote"unbound
c
framework_includesDSet of framework search paths for header files (Apple platform only)"unbound
Å
definesmSet of defines needed to compile this target. Each define is a string. Propagated transitively to dependents."unbound
ã
local_definesqSet of defines needed to compile this target. Each define is a string. Not propagated transitively to dependents."unboundCompilationContext"*Creates a <code>CompilationContext</code>.ÿ
create_compilation_outputsï
y
objectsQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>List of object files."None
Å
pic_objectsQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>List of pic object files."NoneCcCompilationOutputs""Create compilation outputs object.ı
create_compile_variablesß
O
cc_toolchainInfo7cc_toolchain for which we are creating build variables.(
ò
feature_configurationW<a class="anchor" href="../builtins/FeatureConfiguration.html">FeatureConfiguration</a>$Feature configuration to be queried.(
…
source_fileâ<a class="anchor" href="../builtins/File.html">File</a>; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>ßOptional source file path for the compilation. Please prefer passing source_file here over appending it to the end of the command line generated from cc_common.get_memory_inefficient_command_line, as then it's in the power of the toolchain author to properly specify and position compiler flags."None
»
output_fileâ<a class="anchor" href="../builtins/File.html">File</a>; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>¶Optional output file path of the compilation. Please prefer passing output_file here over appending it to the end of the command line generated from cc_common.get_memory_inefficient_command_line, as then it's in the power of the toolchain author to properly specify and position compiler flags."None
’
user_compile_flagsâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>-List of additional compilation flags (copts)."None
é
include_directoriesQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>Depset of include directories."None
ö
quote_include_directoriesQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>$Depset of quote include directories."None
ú
system_include_directoriesQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>%Depset of system include directories."None
¢
framework_include_directoriesQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>(Depset of framework include directories."None
ê
preprocessor_definesQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>Depset of preprocessor defines."None
z
thinlto_indexM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>LTO index file path."None
ù
thinlto_input_bitcode_fileM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>*Bitcode file that is input to LTO backend."None
ù
thinlto_output_object_fileM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>*Object file that is output by LTO backend."None
â
use_pic3<a class="anchor" href="../core/bool.html">bool</a>BWhen true the compilation will generate position independent code."False
]
add_legacy_cxx_options3<a class="anchor" href="../core/bool.html">bool</a>Unused."False
í
variables_extension3<a class="anchor" href="../core/dict.html">dict</a>=A dictionary of additional variables used by compile actions."unbound	Variables"/Returns variables used for compilation actions.
create_library_to_link±
)
actions<code>actions</code> object.(
P
feature_configuration1<code>feature_configuration</code> to be queried."None
G
cc_toolchain1<code>CcToolchainInfo</code> provider to be used."None
ò
static_libraryM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>1<code>File</code> of static library to be linked."None
†
pic_static_libraryM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>5<code>File</code> of pic static library to be linked."None
˘
dynamic_libraryM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>ê<code>File</code> of dynamic library to be linked. Always used for runtime and used for linking if <code>interface_library</code> is not passed."None
û
interface_libraryM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>4<code>File</code> of interface library to be linked."None
•
pic_objectss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>sExperimental, do not use"unbound
°
objectss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>sExperimental, do not use"unbound
í

alwayslink3<a class="anchor" href="../core/bool.html">bool</a>HWhether to link the static library/objects in the --whole_archive block."False
À
dynamic_library_symlink_path7<a class="anchor" href="../core/string.html">string</a>nOverride the default path of the dynamic library link in the solib directory. Empty string to use the default."''
œ
interface_library_symlink_path7<a class="anchor" href="../core/string.html">string</a>pOverride the default path of the interface library link in the solib directory. Empty string to use the default."''LibraryToLink""Creates <code>LibraryToLink</code>≠
create_link_variables·
O
cc_toolchainInfo7cc_toolchain for which we are creating build variables.(
ò
feature_configurationW<a class="anchor" href="../builtins/FeatureConfiguration.html">FeatureConfiguration</a>$Feature configuration to be queried.(
ß
library_search_directories;<a class="anchor" href="../builtins/depset.html">depset</a>HDepset of directories where linker will look for libraries at link time."[]
≠
"runtime_library_search_directories;<a class="anchor" href="../builtins/depset.html">depset</a>FDepset of directories where loader will look for libraries at runtime."[]
y
user_link_flags7<a class="anchor" href="../core/list.html">sequence</a>)List of additional link flags (linkopts)."[]
/
output_fileOptional output file path."None
-

param_fileOptional param file path."None
∫
is_using_linker3<a class="anchor" href="../core/bool.html">bool</a>ÎTrue when using linker, False when archiver. Caller is responsible for keeping this in sync with action name used (is_using_linker = True for linking executable or dynamic library, is_using_linker = False for archiving static library)."True
¢
is_linking_dynamic_library3<a class="anchor" href="../core/bool.html">bool</a>«True when creating dynamic library, False when executable or static library. Caller is responsible for keeping this in sync with action name used. This field will be removed once b/65151735 is fixed."False
Ë
must_keep_debug3<a class="anchor" href="../core/bool.html">bool</a>ôWhen set to False, bazel will expose 'strip_debug_symbols' variable, which is usually used to use the linker to strip debug symbols from the output file."True
á
use_test_only_flags3<a class="anchor" href="../core/bool.html">bool</a>4When set to true, 'is_cc_test' variable will be set."False
\
is_static_linking_mode3<a class="anchor" href="../core/bool.html">bool</a>Unused."True	Variables"0Returns link variables used for linking actions.Ä	
create_linker_input√
â
owner9<a class="anchor" href="../builtins/Label.html">Label</a>CThe label of the target that produced all files used in this input.(
â
	librariesQ<code>None</code>; or <a class="anchor" href="../builtins/depset.html">depset</a>#List of <code>LibraryToLink</code>."None
Á
user_link_flagsÖ<code>None</code>; or <a class="anchor" href="../builtins/depset.html">depset</a> of <a class="anchor" href="../core/string.html">string</a>s; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s≈User link flags passed as strings. Accepts either [String], [[String]] or depset(String). The latter is discouraged as it's only kept for compatibility purposes, the depset is flattened. If you want to propagate user_link_flags via unflattened depsets() wrap them in a LinkerInput so that they are not flattened till the end."None
±
additional_inputsQ<code>None</code>; or <a class="anchor" href="../builtins/depset.html">depset</a>CFor additional inputs to the linking action, e.g.: linking scripts."NoneLinkerInput"#Creates a <code>LinkerInput</code>.»
create_linking_contextÖ
s
linker_inputs;<a class="anchor" href="../builtins/depset.html">depset</a>#Depset of <code>LinkerInput</code>.(LinkingContext"&Creates a <code>LinkingContext</code>.ü
/create_linking_context_from_compilation_outputs…
h
actions=<a class="anchor" href="../builtins/actions.html">actions</a><code>actions</code> object.(
í
name7<a class="anchor" href="../core/string.html">string</a>OThis is used for naming the output artifacts of actions created by this method.(
•
feature_configurationW<a class="anchor" href="../builtins/FeatureConfiguration.html">FeatureConfiguration</a>1<code>feature_configuration</code> to be queried.(
I
cc_toolchainInfo1<code>CcToolchainInfo</code> provider to be used.(
Ç
language7<a class="anchor" href="../core/string.html">string</a>6Only C++ supported for now. Do not use this parameter."'c++'
Ñ
disallow_static_libraries3<a class="anchor" href="../core/bool.html">bool</a>+Whether static libraries should be created."False
Ñ
disallow_dynamic_library3<a class="anchor" href="../core/bool.html">bool</a>,Whether a dynamic library should be created."False
¶
compilation_outputsW<a class="anchor" href="../builtins/CcCompilationOutputs.html">CcCompilationOutputs</a>4Compilation outputs containing object files to link.(
◊
linking_contexts7<a class="anchor" href="../core/list.html">sequence</a>ÖLibraries from dependencies. These libraries will be linked into the output artifact of the link() call, be it a binary or a library."[]
s
user_link_flags7<a class="anchor" href="../core/list.html">sequence</a>#Additional list of linking options."[]
w

alwayslink3<a class="anchor" href="../core/bool.html">bool</a>-Whether this library should always be linked."False
ï
additional_inputs7<a class="anchor" href="../core/list.html">sequence</a>CFor additional inputs to the linking action, e.g.: linking scripts."[]
±
variables_extension3<a class="anchor" href="../core/dict.html">dict</a>\Additional variables to pass to the toolchain configuration when creating link command line."unboundtuple"üShould be used for creating library rules that can propagate information downstream in order to be linked later by a top level rule that does transitive linking to create an executable or a dynamic library. Returns tuple of (<code>CcLinkingContext</code>, <code>CcLinkingOutputs</code>).¡
create_lto_compilation_context
f
objects3<a class="anchor" href="../core/dict.html">dict</a>"map of full object to index object"{}LtoCompilationContext"Create LTO compilation contextﬂ
%do_not_use_tools_cpp_compiler_presentNoneType"´Do not use this field, its only purpose is to help with migration from config_setting.values{'compiler') to config_settings.flag_values{'@bazel_tools//tools/cpp:compiler'}—
get_environment_variables¯
ò
feature_configurationW<a class="anchor" href="../builtins/FeatureConfiguration.html">FeatureConfiguration</a>$Feature configuration to be queried.(
Ü
action_name7<a class="anchor" href="../core/string.html">string</a>ªName of the action. Has to be one of the names in @bazel_tools//tools/build_defs/cc:action_names.bzl (https://github.com/bazelbuild/bazel/blob/master/tools/build_defs/cc/action_names.bzl)(
L
	variables	Variables2Build variables to be used for template expansion.(dict"9Returns environment variables to be set for given action.ˇ
get_execution_requirementsÆ
ò
feature_configurationW<a class="anchor" href="../builtins/FeatureConfiguration.html">FeatureConfiguration</a>$Feature configuration to be queried.(
Ü
action_name7<a class="anchor" href="../core/string.html">string</a>ªName of the action. Has to be one of the names in @bazel_tools//tools/build_defs/cc:action_names.bzl (https://github.com/bazelbuild/bazel/blob/master/tools/build_defs/cc/action_names.bzl)(sequence"0Returns execution requirements for given action.û
#get_memory_inefficient_command_line˝
ò
feature_configurationW<a class="anchor" href="../builtins/FeatureConfiguration.html">FeatureConfiguration</a>$Feature configuration to be queried.(
Ü
action_name7<a class="anchor" href="../core/string.html">string</a>ªName of the action. Has to be one of the names in @bazel_tools//tools/build_defs/cc:action_names.bzl (https://github.com/bazelbuild/bazel/blob/master/tools/build_defs/cc/action_names.bzl)(
M
	variables	Variables3Build variables to be used for template expansions.(sequence"ˆReturns flattened command line flags for given action, using given variables for expansion. Flattens nested sets and ideally should not be used, or at least should not outlive analysis. Work on memory efficient function returning Args is ongoing.È
get_tool_for_action¨
ò
feature_configurationW<a class="anchor" href="../builtins/FeatureConfiguration.html">FeatureConfiguration</a>$Feature configuration to be queried.(
Ü
action_name7<a class="anchor" href="../core/string.html">string</a>ªName of the action. Has to be one of the names in @bazel_tools//tools/build_defs/cc:action_names.bzl (https://github.com/bazelbuild/bazel/blob/master/tools/build_defs/cc/action_names.bzl)(string"#Returns tool path for given action.Ÿ

is_enabledÇ
ò
feature_configurationW<a class="anchor" href="../builtins/FeatureConfiguration.html">FeatureConfiguration</a>$Feature configuration to be queried.(
_
feature_name7<a class="anchor" href="../core/string.html">string</a>Name of the feature.(bool"FReturns True if given feature is enabled in the feature configuration.ä
link’
h
actions=<a class="anchor" href="../builtins/actions.html">actions</a><code>actions</code> object.(
í
name7<a class="anchor" href="../core/string.html">string</a>OThis is used for naming the output artifacts of actions created by this method.(
•
feature_configurationW<a class="anchor" href="../builtins/FeatureConfiguration.html">FeatureConfiguration</a>1<code>feature_configuration</code> to be queried.(
I
cc_toolchainInfo1<code>CcToolchainInfo</code> provider to be used.(
Ç
language7<a class="anchor" href="../core/string.html">string</a>6Only C++ supported for now. Do not use this parameter."'c++'
Ü
output_type7<a class="anchor" href="../core/string.html">string</a>0Can be either 'executable' or 'dynamic_library'."'executable'
ã
link_deps_statically3<a class="anchor" href="../core/bool.html">bool</a>8True to link dependencies statically, False dynamically."True
¿
compilation_outputsm<a class="anchor" href="../builtins/CcCompilationOutputs.html">CcCompilationOutputs</a>; or <code>None</code>4Compilation outputs containing object files to link."None
±
linking_contexts7<a class="anchor" href="../core/list.html">sequence</a>`Linking contexts from dependencies to be linked into the linking context generated by this rule."[]
r
user_link_flags7<a class="anchor" href="../core/list.html">sequence</a>"Additional list of linker options."[]
Æ
stamp1<a class="anchor" href="../core/int.html">int</a>ÓWhether to include build information in the linked executable, if output_type is 'executable'. If 1, build information is always included. If 0 (the default build information is always excluded. If -1, uses the default behavior, which may be overridden by the --[no]stamp flag. This should be unset (or set to 0) when generating the executable output for test rules."0
’
additional_inputsw<a class="anchor" href="../core/list.html">sequence</a>; or <a class="anchor" href="../builtins/depset.html">depset</a>CFor additional inputs to the linking action, e.g.: linking scripts."[]
ë
additional_outputs7<a class="anchor" href="../core/list.html">sequence</a>>For additional outputs to the linking action, e.g.: map files."[]
™
variables_extension3<a class="anchor" href="../core/dict.html">dict</a>ZAdditional variables to pass to the toolchain configuration when create link command line."{}CcLinkingOutputs"*Should be used for C++ transitive linking.ﬂ
merge_cc_infosú
»
direct_cc_infos7<a class="anchor" href="../core/list.html">sequence</a>xList of <code>CcInfo</code>s to be merged, whose headers will be exported by the direct fields in the returned provider."[]
≈
cc_infos7<a class="anchor" href="../core/list.html">sequence</a>|List of <code>CcInfo</code>s to be merged, whose headers will not be exported by the direct fields in the returned provider."[]unknown".Merges multiple <code>CcInfo</code>s into one.‹
merge_compilation_contextsÄ
È
compilation_contexts7<a class="anchor" href="../core/list.html">sequence</a>ìList of <code>CompilationContexts</code>s to be merged. The headers of each context will be exported by the direct fields in the returned provider."[]CompilationContext";Merges multiple <code>CompilationContexts</code>s into one.£
merge_compilation_outputsj
R
compilation_outputs7<a class="anchor" href="../core/list.html">sequence</a>"[]CcCompilationOutputs"Merge compilation outputs.DUtilities for C++ compilation, linking, and command line generation.
 "
configº
booló
Ü
flag3<a class="anchor" href="../core/bool.html">bool</a>BWhether or not this build setting is callable on the command line."FalseBuildSetting"A bool-typed build settingŒ
exec£
â

exec_groupM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>•The name of the exec group whose execution platform this transition will use. If not provided, this exec transition will use the target's default execution platform."NoneExecTransitionFactory" Creates an execution transition.ø
intó
Ü
flag3<a class="anchor" href="../core/bool.html">bool</a>BWhether or not this build setting is callable on the command line."FalseBuildSetting"An integer-typed build settingÊ
none
transition"œCreates a transition which removes all configuration, unsetting all flags. Intended for the case where a dependency is data-only and contains no code that needs to be built, but should only be analyzed once.⁄
string±
Ü
flag3<a class="anchor" href="../core/bool.html">bool</a>BWhether or not this build setting is callable on the command line."False
ó
allow_multiple3<a class="anchor" href="../core/bool.html">bool</a>»Deprecated, use a <code>string_list</code> setting with <code>repeatable = True</code> instead. If set, this flag is allowed to be set multiple times on the command line. The Value of the flag as accessed in transitions and build setting implementation function will be a list of strings. Insertion order and repeated values are both maintained. This list can be post-processed in the build setting implementation function if different behavior is desired."FalseBuildSetting"A string-typed build settingÔ
string_list‘
Ü
flag3<a class="anchor" href="../core/bool.html">bool</a>BWhether or not this build setting is callable on the command line."False
∫

repeatable3<a class="anchor" href="../core/bool.html">bool</a>ÔIf set, instead of expecting a comma-separated value, this flag is allowed to be set multiple times on the command line with each individual value treated as a single string to add to the list value. Insertion order and repeated values are both maintained. This list can be post-processed in the build setting implementation function if different behavior is desired."FalseBuildSetting"àA string list-typed build setting. On the command line pass a list using comma-separated value like <code>--//my/setting=foo,bar</code>.ﬂ

string_setå
Ü
flag3<a class="anchor" href="../core/bool.html">bool</a>BWhether or not this build setting is callable on the command line."False
Ú

repeatable3<a class="anchor" href="../core/bool.html">bool</a>ßIf set, instead of expecting a comma-separated value, this flag is allowed to be set multiple times on the command line with each individual value treated as a single string to add to the set value. Only a single instance of repeated values is maintained and the insertion order does not matter."FalseBuildSetting"¡A string set-typed build setting. The value of this setting will be a <a href='https://bazel.build/rules/lib/core/set'>set</a> of strings in Starlark. On the command line, pass a set using a comma-separated value like <code>--//my/setting=foo,bar</code>.<p>Unlike with a <code>string_list</code>, the order of the elements doesn't matter and only a single instance of each element is maintained. This is recommended over <code>string_list</code> for flags where these properties are not needed as it can improve build performance by avoiding unnecessary configurations forking.Å
target
transition"ËCreates a target transition. This is a no-op transition intended for the case where a transition object is needed, but doesn't want to actually change anything. Equivalent to <code>cfg = "target"</code> in <code>attr.label()</code>.œThis is a top-level module for creating configuration transitions and build setting descriptors which describe what kind of build setting (if any) a rule is. <p>ex: the following rule is marked as a build setting by setting the <code>build_setting</code> parameter of the <code>rule()</code> function. Specifically it is a build setting of type <code>int</code> and is a <code>flag</code> which means this build setting is callable on the command line.<br><pre class=language-python>  my_rule = rule(
    implementation = _impl,
    build_setting = config.int(flag = True),
    ...
  )</pre>
¿
config_commonj
FeatureFlagInfoProvider"MThe key used to retrieve the provider containing config_feature_flag's value.˚
toolchain_type∂
•
nameu<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>$The toolchain type that is required.(
|
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>4Whether the toolchain type is mandatory or optional."Truetoolchain_type"0Declare a rule's dependency on a toolchain type.EFunctions for Starlark to interact with Blaze's configurability APIs.
ÿ
coverage_commoná
instrumented_files_infoû

Q
ctx5<a class="anchor" href="../builtins/ctx.html">ctx</a>The rule context.(
û
source_attributes7<a class="anchor" href="../core/list.html">sequence</a>LA list of attribute names which contain source files processed by this rule."[]
¿
dependency_attributes7<a class="anchor" href="../core/list.html">sequence</a>jA list of attribute names which might provide runtime dependencies (either code dependencies or runfiles)."[]
®

extensionsâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>áFile extensions used to filter files from source_attributes. For example, 'js'. If not provided (or None), then all files from source_attributes will be added to instrumented files, if an empty list is provided, then no files from source attributes will be added."None
Ù
metadata_filess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>siAdditional files required to generate coverage LCOV files after code execution. e.g. .gcno files for C++."[]
´
baseline_coverage_filesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <code>None</code>"NoneInstrumentedFilesInfo" Creates a new <a class="anchor" href="../providers/InstrumentedFilesInfo.html">InstrumentedFilesInfo</a> instance. Use this provider to communicate coverage-related attributes of the current build rule.;Helper functions to access coverage-related infrastructure.
µC
java_commonT
BootClassPathInfoProvider"5The provider used to supply bootclasspath information
JavaRuntimeInfoProvider"bThe key used to retrieve the provider that contains information about the Java runtime being used.É
JavaToolchainInfoProvider"dThe key used to retrieve the provider that contains information about the Java toolchain being used.ó)
compileπ'
Q
ctx5<a class="anchor" href="../builtins/ctx.html">ctx</a>The rule context.(
Î
source_jarss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>scA list of the jars to be compiled. At least one of source_jars or source_files should be specified."[]
˘
source_filess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>spA list of the Java source files to be compiled. At least one of source_jars or source_files should be specified."[]
C
output7<a class="anchor" href="../builtins/File.html">File</a>(
≠
output_source_jarM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>CThe output source jar. Defaults to `{output_jar}-src.jar` if unset."None
´

javac_optss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s$A list of the desired javac options."[]
ú
depsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>sA list of dependencies."[]
¨
runtime_depsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>sA list of runtime dependencies."[]
ö
exportsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>sA list of exports."[]
ó
pluginsÛ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>s; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>sA list of plugins."[]
©
exported_pluginsÛ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>s; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>sA list of exported plugins."[]
“
native_librariesx<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../providers/CcInfo.html">CcInfo</a>s@CC native library dependencies that are needed for this library."[]
ô
&annotation_processor_additional_inputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>svA list of inputs that the Java compilation action will take in addition to the Java sources for annotation processing."[]
õ
'annotation_processor_additional_outputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>swA list of outputs that the Java compilation action will output in addition to the class jar from annotation processing."[]
¿
strict_deps7<a class="anchor" href="../core/string.html">string</a>ÓA string that specifies how to handle strict deps. Possible values: 'OFF', 'ERROR', 'WARN' and 'DEFAULT'. For more details see <a href="/docs/user-manual#flag--strict_java_deps"><code>--strict_java_deps<code> flag</a>. By default 'ERROR'."'ERROR'
Y
java_toolchainInfo?A JavaToolchainInfo to be used for this compilation. Mandatory.(
°
bootclasspathâA BootClassPathInfo to be used for this compilation. If present, overrides the bootclasspath associated with the provided java_toolchain."None
Ö

sourcepaths<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s"[]
Ñ
	resourcess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s"[]
à
resource_jarss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s"[]
é
classpath_resourcess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s"[]
G
	neverlink3<a class="anchor" href="../core/bool.html">bool</a>"False
Û
enable_annotation_processing3<a class="anchor" href="../core/bool.html">bool</a>óDisables annotation processing in this compilation, causing any annotation processors provided in plugins or in exported_plugins of deps to be ignored."True
Õ
enable_compile_jar_action3<a class="anchor" href="../core/bool.html">bool</a>ÙEnables header compilation or ijar creation. If set to False, it forces use of the full class jar in the compilation classpaths of any dependants. Doing so is intended for use by non-library targets such as binaries that do not have dependants."True
¬
add_exportss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s:Allow this library to access the given <module>/<package>."[]
Õ
	add_openss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sGAllow this library to reflectively access the given <module>/<package>."[]struct"œCompiles Java source files/jars from the implementation of a Starlark rule and returns a provider that represents the results of the compilation and can be added to the set of providers emitted by this rule.
merge≤
ß
	providersw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>sThe list of providers to merge.(struct"2Merges the given providers into a single JavaInfo.’
pack_sources∂
W
actions=<a class="anchor" href="../builtins/actions.html">actions</a>ctx.actions(
Ä
output_source_jarM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>The output source jar."None
¡
sourcess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s=A list of Java source files to be packed into the source jar."[]
ø
source_jarss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s7A list of source jars to be packed into the source jar."[]
L
java_toolchainInfo2A JavaToolchainInfo to used to find the ijar tool.(File"ãPacks sources and source jars into a single source jar file. The return value is typically passed to<p><code><a class="anchor" href="../providers/JavaInfo.html">JavaInfo</a>#source_jar</code></p>.At least one of parameters output_jar or output_source_jar is required.Î
run_ijar†
W
actions=<a class="anchor" href="../builtins/actions.html">actions</a>ctx.actions(
Y
jar7<a class="anchor" href="../builtins/File.html">File</a>The jar to run ijar on.(
ï
target_labelO<a class="anchor" href="../builtins/Label.html">Label</a>; or <code>None</code>≠A target label to stamp the jar with. Used for <code>add_dep</code> support. Typically, you would pass <code>ctx.label</code> to stamp the jar with the current rule's label."None
L
java_toolchainInfo2A JavaToolchainInfo to used to find the ijar tool.(File"ªRuns ijar on a jar, stripping it of its method bodies. This helps reduce rebuilding of dependent jars during any recompiles consisting only of simple changes to method implementations. The return value is typically passed to <code><a class="anchor" href="../providers/JavaInfo.html">JavaInfo</a>#compile_jar</code>.¿
	stamp_jarê
W
actions=<a class="anchor" href="../builtins/actions.html">actions</a>ctx.actions(
^
jar7<a class="anchor" href="../builtins/File.html">File</a>The jar to run stamp_jar on.(
˚
target_label9<a class="anchor" href="../builtins/Label.html">Label</a>≠A target label to stamp the jar with. Used for <code>add_dep</code> support. Typically, you would pass <code>ctx.label</code> to stamp the jar with the current rule's label.(
Q
java_toolchainInfo7A JavaToolchainInfo to used to find the stamp_jar tool.(File"üStamps a jar with a target label for <code>add_dep</code> support. The return value is typically passed to <code><a class="anchor" href="../providers/JavaInfo.html">JavaInfo</a>#compile_jar</code>. Prefer to use <code><a class="anchor" href="#run_ijar">run_ijar</a></code> when possible.3Utilities for Java compilation support in Starlark.
Ì‹
nativeÆ
existing_rulee
Z
name7<a class="anchor" href="../core/string.html">string</a>The name of the target.(unknown"≥Returns an immutable dict-like object that describes the attributes of a rule instantiated in this thread's package, or <code>None</code> if no rule instance of that name exists.<p>Here, an <em>immutable dict-like object</em> means a deeply immutable object <code>x</code> supporting dict-like iteration, <code>len(x)</code>, <code>name in x</code>, <code>x[name]</code>, <code>x.get(name)</code>, <code>x.items()</code>, <code>x.keys()</code>, and <code>x.values()</code>.<p>The result contains an entry for each attribute, with the exception of private ones (whose names do not start with a letter) and a few unrepresentable legacy attribute types. In addition, the dict contains entries for the rule instance's <code>name</code> and <code>kind</code> (for example, <code>'cc_binary'</code>).<p>The values of the result represent attribute values as follows:<ul><li>Attributes of type str, int, and bool are represented as is.</li><li>Labels are converted to strings of the form <code>':foo'</code> for targets in the same package or <code>'//pkg:name'</code> for targets in a different package.</li><li>Lists are represented as tuples, and dicts are converted to new, mutable dicts. Their elements are recursively converted in the same fashion.</li><li><code>select</code> values are returned with their contents transformed as described above.</li><li>Attributes for which no value was specified during rule instantiation and whose default value is computed are excluded from the result. (Computed defaults cannot be computed until the analysis phase.).</li></ul><p>If possible, use this function only in <a href="https://bazel.build/extending/macros#finalizers">implementation functions of rule finalizer symbolic macros</a>. Use of this function in other contexts is not recommened, and will be disabled in a future Bazel release; it makes <code>BUILD</code> files brittle and order-dependent. Also, beware that it differs subtly from the two other conversions of rule attribute values from internal form to Starlark: one used by computed defaults, the other used by <code>ctx.attr.foo</code>.(ã
existing_rules	unknown"ÎReturns an immutable dict-like object describing the rules so far instantiated in this thread's package. Each entry of the dict-like object maps the name of the rule instance to the result that would be returned by <code>existing_rule(name)</code>.<p>Here, an <em>immutable dict-like object</em> means a deeply immutable object <code>x</code> supporting dict-like iteration, <code>len(x)</code>, <code>name in x</code>, <code>x[name]</code>, <code>x.get(name)</code>, <code>x.items()</code>, <code>x.keys()</code>, and <code>x.values()</code>.<p>If possible, use this function only in <a href="https://bazel.build/extending/macros#finalizers">implementation functions of rule finalizer symbolic macros</a>. Use of this function in other contexts is not recommened, and will be disabled in a future Bazel release; it makes <code>BUILD</code> files brittle and order-dependent.(„
exports_filesı
õ
srcss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sThe list of files to export.(
ê

visibilityM<a class="anchor" href="../core/list.html">sequence</a>; or <code>None</code>¨A visibility declaration can to be specified. The files will be visible to the targets specified. If no visibility is specified, the files will be visible to every package."None
∑
licensesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>Licenses to be specified."NoneNoneType"XSpecifies a list of files belonging to this package that are exported to other packages.(í	
glob‚
©
includes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s%The list of glob patterns to include."[]
©
excludes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s%The list of glob patterns to exclude."[]
z
exclude_directories1<a class="anchor" href="../core/int.html">int</a>-A flag whether to exclude directories or not."1
Å
allow_emptyËWhether we allow glob patterns to match nothing. If `allow_empty` is False, each individual include pattern must match something and also the final result must be non-empty (after the matches of the `exclude` patterns are excluded)."unboundsequence"¢Glob returns a new, mutable, sorted list of every file in the current package that:<ul>
<li>Matches at least one pattern in <code>include</code>.</li>
<li>Does not match any of the patterns in <code>exclude</code> (default <code>[]</code>).</li></ul>
If the <code>exclude_directories</code> argument is enabled (set to <code>1</code>), files of type directory will be omitted from the results (default <code>1</code>).(˛
module_namestring"‚The name of the Bazel module associated with the repo this package is in. If this package is from a repo defined in WORKSPACE instead of MODULE.bazel, this is empty. For repos generated by module extensions, this is the name of the module hosting the extension. It's the same as the <code>module.name</code> field seen in <code>module_ctx.modules</code>.(ä
module_versionstring"ÎThe version of the Bazel module associated with the repo this package is in. If this package is from a repo defined in WORKSPACE instead of MODULE.bazel, this is empty. For repos generated by module extensions, this is the version of the module hosting the extension. It's the same as the <code>module.version</code> field seen in <code>module_ctx.modules</code>.(Ì
package_default_visibilitylist"ƒReturns the default visibility of the package being evaluated. This is the value of the <code>default_visibility</code> parameter of <code>package()</code>, extended to include the package itself.(Ñ
package_group·
a
name7<a class="anchor" href="../core/string.html">string</a>The unique name for this rule.(
∂
packagess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s1A complete enumeration of packages in this group."[]
∏
includess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s3Other package groups that are included in this one."[]NoneType"åThis function defines a set of packages and assigns a label to the group. The label can be referenced in <code>visibility</code> attributes.(è
package_namestring"ÚThe name of the package being evaluated, without the repository name. For example, in the BUILD file <code>some/package/BUILD</code>, its value will be <code>some/package</code>. If the BUILD file calls a function defined in a .bzl file, <code>package_name()</code> will match the caller BUILD file package. The value will always be an empty string for the root package.(π
package_relative_labelÂ
€
inputu<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>YThe input label string or Label object. If a Label object is passed, it's returned as is.(Label"¥Converts the input string into a <a href='../builtins/Label.html'>Label</a> object, in the context of the package currently being initialized (that is, the <code>BUILD</code> file for which the current macro is executing). If the input is already a <code>Label</code>, it is returned unchanged.<p>This function may only be called while evaluating a BUILD file and the macros it directly or indirectly calls; it may not be called in (for instance) a rule implementation function. <p>The result of this function is the same <code>Label</code> value as would be produced by passing the given string to a label-valued attribute of a target declared in the BUILD file. <p><i>Usage note:</i> The difference between this function and <a href='../builtins/Label.html#Label'>Label()</a></code> is that <code>Label()</code> uses the context of the package of the <code>.bzl</code> file that called it, not the package of the <code>BUILD</code> file. Use <code>Label()</code> when you need to refer to a fixed target that is hardcoded into the macro, such as a compiler. Use <code>package_relative_label()</code> when you need to normalize a label string supplied by the BUILD file to a <code>Label</code> object. (There is no way to convert a string to a <code>Label</code> in the context of a package other than the BUILD file or the calling .bzl file. For that reason, outer macros should always prefer to pass Label objects to inner macros rather than label strings.)(â
	repo_namestring"pThe canonical name of the repository containing the package currently being evaluated, with no leading at-signs.(±
repository_namestring"ë<b>Experimental</b>. This API is experimental and may change at any time. Please do not depend on it. It may be enabled on an experimental basis by setting <code>--+incompatible_enable_deprecated_label_apis</code> <br><strong>Deprecated.</strong> Prefer to use <a href="#repo_name"><code>repo_name</code></a> instead, which doesn't contain the spurious leading at-sign, but behaves identically otherwise.<p>The canonical name of the repository containing the package currently being evaluated, with a single at-sign (<code>@</code>) prefixed. For example, in packages that are called into existence by the WORKSPACE stanza <code>local_repository(name='local', path=...)</code> it will be set to <code>@local</code>. In packages in the main repository, it will be set to <code>@</code>.(ú
subpackages∑
ª
includes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s9The list of glob patterns to include in subpackages scan.(
ø
excludes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s;The list of glob patterns to exclude from subpackages scan."[]
™
allow_empty3<a class="anchor" href="../core/bool.html">bool</a>ﬁWhether we fail if the call returns an empty list. By default empty list indicates potential error in BUILD file where the call to subpackages() is superflous.  Setting to true allows this function to succeed in that case."Falsesequence"–Returns a new mutable list of every direct subpackage of the current package, regardless of file-system directory depth. List returned is sorted and contains the names of subpackages relative to the current package. It is advised to prefer using the methods in bazel_skylib.subpackages module rather than calling this function directly.(ó
	cc_binaryˆ

name(

deps

srcs

data

additional_linker_inputs

args

aspect_hints

compatible_with

	conlyopts

copts
	
cxxopts
	
defines

deprecation


distribs

dynamic_deps

env

exec_compatible_with

exec_group_compatible_with

exec_properties


features


hdrs_check


includes


licenses

link_extra_lib


linkopts


linkshared


linkstatic

local_defines

malloc

module_interfaces
	
nocopts

output_licenses

package_metadata

reexport_deps

restricted_to

stamp

tags

target_compatible_with


testonly


toolchains


visibility

win_def_file

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies

$bzl_load_label"é<p>It produces an executable binary.</p>

<br/>The <code>name</code> of the target should be the same as the name of the
source file that is the main entry point of the application (minus the extension).
For example, if your entry point is in <code>main.cc</code>, then your name should
be <code>main</code>.

<h4>Implicit output targets</h4>
<ul>
<li><code><var>name</var>.stripped</code> (only built if explicitly requested): A stripped
  version of the binary. <code>strip -g</code> is run on the binary to remove debug
  symbols.  Additional strip options can be provided on the command line using
  <code>--stripopt=-foo</code>.</li>
<li><code><var>name</var>.dwp</code> (only built if explicitly requested): If
  <a href="https://gcc.gnu.org/wiki/DebugFission">Fission</a> is enabled: a debug
  information package file suitable for debugging remotely deployed binaries. Else: an
  empty file.</li>
</ul>(€
	cc_import˝

name(

deps

data

hdrs


alwayslink

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


includes

interface_library


linkopts
	
objects

package_metadata

pic_objects

pic_static_library

restricted_to

shared_library

static_library

strip_include_prefix

system_provided

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"À<p>
<code>cc_import</code> rules allows users to import precompiled C/C++ libraries.
</p>

<p>
The following are the typical use cases: <br/>

1. Linking a static library
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  static_library = "libmylib.a",
  # If alwayslink is turned on,
  # libmylib.a will be forcely linked into any binary that depends on it.
  # alwayslink = 1,
)
</code></pre>

2. Linking a shared library (Unix)
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  shared_library = "libmylib.so",
)
</code></pre>

3. Linking a shared library with interface library

<p>On Unix:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  # libmylib.ifso is an interface library for libmylib.so which will be passed to linker
  interface_library = "libmylib.ifso",
  # libmylib.so will be available for runtime
  shared_library = "libmylib.so",
)
</code></pre>

<p>On Windows:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  # mylib.lib is an import library for mylib.dll which will be passed to linker
  interface_library = "mylib.lib",
  # mylib.dll will be available for runtime
  shared_library = "mylib.dll",
)
</code></pre>

4. Linking a shared library with <code>system_provided=True</code>

<p>On Unix:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  interface_library = "libmylib.ifso", # Or we can also use libmylib.so as its own interface library
  # libmylib.so is provided by system environment, for example it can be found in LD_LIBRARY_PATH.
  # This indicates that Bazel is not responsible for making libmylib.so available.
  system_provided = 1,
)
</code></pre>

<p>On Windows:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  # mylib.lib is an import library for mylib.dll which will be passed to linker
  interface_library = "mylib.lib",
  # mylib.dll is provided by system environment, for example it can be found in PATH.
  # This indicates that Bazel is not responsible for making mylib.dll available.
  system_provided = 1,
)
</code></pre>

5. Linking to static or shared library

<p>On Unix:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  static_library = "libmylib.a",
  shared_library = "libmylib.so",
)
</code></pre>

<p>On Windows:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  static_library = "libmylib.lib", # A normal static library
  interface_library = "mylib.lib", # An import library for mylib.dll
  shared_library = "mylib.dll",
)
</code></pre>

<p>The remaining is the same on Unix and Windows:
<pre><code class="lang-starlark">
# first will link to libmylib.a (or libmylib.lib)
cc_binary(
  name = "first",
  srcs = ["first.cc"],
  deps = [":mylib"],
  linkstatic = 1, # default value
)

# second will link to libmylib.so (or libmylib.lib)
cc_binary(
  name = "second",
  srcs = ["second.cc"],
  deps = [":mylib"],
  linkstatic = 0,
)
</code></pre>

<p>
<code>cc_import</code> supports an include attribute. For example:
<pre><code class="lang-starlark">
cc_import(
  name = "curl_lib",
  hdrs = glob(["vendor/curl/include/curl/*.h"]),
  includes = ["vendor/curl/include"],
  shared_library = "vendor/curl/lib/.libs/libcurl.dylib",
)
</code></pre>
</p>(ı;

cc_libraryá

name(

deps

srcs

data

hdrs

additional_compiler_inputs

additional_linker_inputs


alwayslink

aspect_hints

compatible_with

	conlyopts

copts
	
cxxopts
	
defines

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


hdrs_check

implementation_deps

include_prefix


includes


licenses


linkopts

	linkstamp


linkstatic

local_defines

module_interfaces

package_metadata

restricted_to

strip_include_prefix

tags

target_compatible_with


testonly

textual_hdrs


toolchains


visibility

win_def_file

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"⁄5<p>Use <code>cc_library()</code> for C++-compiled libraries.
  The result is  either a <code>.so</code>, <code>.lo</code>,
  or <code>.a</code>, depending on what is needed.
</p>

<p>
  If you build something with static linking that depends on
  a <code>cc_library</code>, the output of a depended-on library rule
  is the <code>.a</code> file. If you specify
   <code>alwayslink=True</code>, you get the <code>.lo</code> file.
</p>

<p>
  The actual output file name is <code>lib<i>foo</i>.so</code> for
  the shared library, where <i>foo</i> is the name of the rule.  The
  other kinds of libraries end with <code>.lo</code> and <code>.a</code>,
  respectively.  If you need a specific shared library name, for
  example, to define a Python module, use a genrule to copy the library
  to the desired name.
</p>

<h4 id="hdrs">Header inclusion checking</h4>

<p>
  All header files that are used in the build must be declared in
  the <code>hdrs</code> or <code>srcs</code> of <code>cc_*</code> rules.
  This is enforced.
</p>

<p>
  For <code>cc_library</code> rules, headers in <code>hdrs</code> comprise the
  public interface of the library and can be directly included both
  from the files in <code>hdrs</code> and <code>srcs</code> of the library
  itself as well as from files in <code>hdrs</code> and <code>srcs</code>
  of <code>cc_*</code> rules that list the library in their <code>deps</code>.
  Headers in <code>srcs</code> must only be directly included from the files
  in <code>hdrs</code> and <code>srcs</code> of the library itself. When
  deciding whether to put a header into <code>hdrs</code> or <code>srcs</code>,
  you should ask whether you want consumers of this library to be able to
  directly include it. This is roughly the same decision as
  between <code>public</code> and <code>private</code> visibility in programming languages.
</p>

<p>
  <code>cc_binary</code> and <code>cc_test</code> rules do not have an exported
  interface, so they also do not have a <code>hdrs</code> attribute. All headers
  that belong to the binary or test directly should be listed in
  the <code>srcs</code>.
</p>

<p>
  To illustrate these rules, look at the following example.
</p>

<pre><code class="lang-starlark">
cc_binary(
    name = "foo",
    srcs = [
        "foo.cc",
        "foo.h",
    ],
    deps = [":bar"],
)

cc_library(
    name = "bar",
    srcs = [
        "bar.cc",
        "bar-impl.h",
    ],
    hdrs = ["bar.h"],
    deps = [":baz"],
)

cc_library(
    name = "baz",
    srcs = [
        "baz.cc",
        "baz-impl.h",
    ],
    hdrs = ["baz.h"],
)
</code></pre>

<p>
  The allowed direct inclusions in this example are listed in the table below.
  For example <code>foo.cc</code> is allowed to directly
  include <code>foo.h</code> and <code>bar.h</code>, but not <code>baz.h</code>.
</p>

<table class="table table-striped table-bordered table-condensed">
  <thead>
    <tr><th>Including file</th><th>Allowed inclusions</th></tr>
  </thead>
  <tbody>
    <tr><td>foo.h</td><td>bar.h</td></tr>
    <tr><td>foo.cc</td><td>foo.h bar.h</td></tr>
    <tr><td>bar.h</td><td>bar-impl.h baz.h</td></tr>
    <tr><td>bar-impl.h</td><td>bar.h baz.h</td></tr>
    <tr><td>bar.cc</td><td>bar.h bar-impl.h baz.h</td></tr>
    <tr><td>baz.h</td><td>baz-impl.h</td></tr>
    <tr><td>baz-impl.h</td><td>baz.h</td></tr>
    <tr><td>baz.cc</td><td>baz.h baz-impl.h</td></tr>
  </tbody>
</table>

<p>
  The inclusion checking rules only apply to <em>direct</em>
  inclusions. In the example above <code>foo.cc</code> is allowed to
  include <code>bar.h</code>, which may include <code>baz.h</code>, which in
  turn is allowed to include <code>baz-impl.h</code>. Technically, the
  compilation of a <code>.cc</code> file may transitively include any header
  file in the <code>hdrs</code> or <code>srcs</code> in
  any <code>cc_library</code> in the transitive <code>deps</code> closure. In
  this case the compiler may read <code>baz.h</code> and <code>baz-impl.h</code>
  when compiling <code>foo.cc</code>, but <code>foo.cc</code> must not
  contain <code>#include "baz.h"</code>. For that to be
  allowed, <code>baz</code> must be added to the <code>deps</code>
  of <code>foo</code>.
</p>

<p>
  Bazel depends on toolchain support to enforce the inclusion checking rules.
  The <code>layering_check</code> feature has to be supported by the toolchain
  and requested explicitly, for example via the
  <code>--features=layering_check</code> command-line flag or the
  <code>features</code> parameter of the
  <a href="#package"><code>package</code></a> function. The toolchains
  provided by Bazel only support this feature with clang on Unix and macOS.
</p>

<h4 id="cc_library_examples">Examples</h4>

<p id="alwayslink_lib_example">
   We use the <code>alwayslink</code> flag to force the linker to link in
   this code although the main binary code doesn't reference it.
</p>

<pre><code class="lang-starlark">
cc_library(
    name = "ast_inspector_lib",
    srcs = ["ast_inspector_lib.cc"],
    hdrs = ["ast_inspector_lib.h"],
    visibility = ["//visibility:public"],
    deps = ["//third_party/llvm/llvm/tools/clang:frontend"],
    # alwayslink as we want to be able to call things in this library at
    # debug time, even if they aren't used anywhere in the code.
    alwayslink = 1,
)
</code></pre>


<p>The following example comes from
   <code>third_party/python2_4_3/BUILD</code>.
   Some of the code uses the <code>dl</code> library (to load
   another, dynamic library), so this
   rule specifies the <code>-ldl</code> link option to link the
   <code>dl</code> library.
</p>

<pre><code class="lang-starlark">
cc_library(
    name = "python2_4_3",
    linkopts = [
        "-ldl",
        "-lutil",
    ],
    deps = ["//third_party/expat"],
)
</code></pre>

<p>The following example comes from <code>third_party/kde/BUILD</code>.
   We keep pre-built <code>.so</code> files in the depot.
   The header files live in a subdirectory named <code>include</code>.
</p>

<pre><code class="lang-starlark">
cc_library(
    name = "kde",
    srcs = [
        "lib/libDCOP.so",
        "lib/libkdesu.so",
        "lib/libkhtml.so",
        "lib/libkparts.so",
        <var>...more .so files...</var>,
    ],
    includes = ["include"],
    deps = ["//third_party/X11"],
)
</code></pre>

<p>The following example comes from <code>third_party/gles/BUILD</code>.
   Third-party code often needs some <code>defines</code> and
   <code>linkopts</code>.
</p>

<pre><code class="lang-starlark">
cc_library(
    name = "gles",
    srcs = [
        "GLES/egl.h",
        "GLES/gl.h",
        "ddx.c",
        "egl.c",
    ],
    defines = [
        "USE_FLOAT",
        "__GL_FLOAT",
        "__GL_COMMON",
    ],
    linkopts = ["-ldl"],  # uses dlopen(), dl library
    deps = [
        "es",
        "//third_party/X11",
    ],
)
</code></pre>(Ô,
cc_shared_library˛

name(

deps

additional_linker_inputs

aspect_hints

compatible_with

deprecation

dynamic_deps

exec_compatible_with

exec_group_compatible_with

exec_properties
=
;experimental_disable_topo_sort_do_not_use_remove_before_7_0

exports_filter


features

package_metadata

restricted_to

roots

shared_lib_name

static_deps

tags

target_compatible_with


testonly


toolchains

user_link_flags


visibility

win_def_file

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"÷'<p>It produces a shared library.</p>

<h4 id="cc_shard_library_examples">Example</h4>

<pre class="code">
cc_shared_library(
    name = "foo_shared",
    deps = [
        ":foo",
    ],
    dynamic_deps = [
        ":bar_shared",
    ],
    additional_linker_inputs = [
        ":foo.lds",
    ],
    user_link_flags = [
        "-Wl,--version-script=$(location :foo.lds)",
    ],
)
cc_library(
    name = "foo",
    srcs = ["foo.cc"],
    hdrs = ["foo.h"],
    deps = [
        ":bar",
        ":baz",
    ],
)
cc_shared_library(
    name = "bar_shared",
    shared_lib_name = "bar.so",
    deps = [":bar"],
)
cc_library(
    name = "bar",
    srcs = ["bar.cc"],
    hdrs = ["bar.h"],
)
cc_library(
    name = "baz",
    srcs = ["baz.cc"],
    hdrs = ["baz.h"],
)
</pre>

<p>In the example <code>foo_shared</code> statically links <code>foo</code>
and <code>baz</code>, the latter being a transitive dependency. It doesn't
link <code>bar</code> because it is already provided dynamically by the
<code>dynamic_dep</code> <code>bar_shared</code>.</p>

<p><code>foo_shared</code> uses a linker script *.lds file to control which
symbols should be exported. The <code>cc_shared_library</code> rule logic does
not control which symbols get exported, it only uses what is assumed to be
exported to give errors during analysis phase if two shared libraries export the
same targets.</p>

<p>Every direct dependency of <code>cc_shared_library</code> is assumed to be
exported. Therefore, Bazel assumes during analysis that <code>foo</code> is being
exported by <code>foo_shared</code>. <code>baz</code> is not assumed to be exported
by <code>foo_shared</code>. Every target matched by the <code>exports_filter</code>
is also assumed to be exported.</p>

<p>Every single <code>cc_library</code> in the example should appear at most in one
<code>cc_shared_library</code>. If we wanted to link <code>baz</code> also into
<code>bar_shared</code> we would need to add
<code>tags = ["LINKABLE_MORE_THAN_ONCE"]</code> to <code>baz</code>.</p>

<p>Due to the <code>shared_lib_name</code> attribute, the file produced by
<code>bar_shared</code> will have the name <code>bar.so</code> as opposed
to the name <code>libbar.so</code> that it would have by default on Linux.</p>

<h4 id="cc_shard_library_examples">Errors</h4>
<h5><code>Two shared libraries in dependencies export the same symbols.</code></h5>

<p>This will happen whenever you are creating a target with two different
<code>cc_shared_library</code> dependencies that export the same target. To fix this
you need to stop the libraries from being exported in one of the
<code>cc_shared_library</code> dependencies.</p>

<h5><code>Two shared libraries in dependencies link the same library statically</code></h5>

<p>This will happen whenever you are creating a new <code>cc_shared_library</code> with two
different <code>cc_shared_library</code> dependencies that link the same target statically.
Similar to the error with exports.</p>

<p>One way to fix this is to stop linking the library into one of the
<code>cc_shared_library</code> dependencies. At the same time, the one that still links it
needs to export the library so that the one not linking it keeps visibility to
the symbols. Another way is to pull out a third library that exports the target.
A third way is to tag the culprit <code>cc_library</code> with <code>LINKABLE_MORE_THAN_ONCE</code>
but this fix should be rare and you should absolutely make sure that the
<code>cc_library</code> is indeed safe to link more than once.</p>

<h5><code>'//foo:foo' is already linked statically in '//bar:bar' but not exported`</code></h5>

<p>This means that a library in the transitive closure of your <code>deps</code> is reachable
without going through one of the <code>cc_shared_library</code> dependencies but is already
linked into a different <code>cc_shared_library</code> in <code>dynamic_deps</code> and is not
exported.</p>

<p>The solution is to export it from the <code>cc_shared_library</code> dependency or pull out
a third <code>cc_shared_library</code> that exports it.</p>

<h5><code>Do not place libraries which only contain a precompiled dynamic library in deps.
</code></h5>

<p>If you have a precompiled dynamic library, this doesn't need to and cannot be
linked statically into the current <code>cc_shared_library</code> target that you are
currently creating. Therefore, it doesn't belong in <code>deps</code> of the
<code>cc_shared_library</code>. If this precompiled dynamic library is a dependency of one
of your <code>cc_libraries</code>, then the <code>cc_library</code> needs to depend on it
directly.</p>

<h5><code>Trying to export a library already exported by a different shared library</code></h5>

<p>You will see this error if on the current rule you are claiming to export a
target that is already being exported by one of your dynamic dependencies.</p>

<p>To fix this, remove the target from <code>deps</code> and just rely on it from the dynamic
dependency or make sure that the <code>exports_filter</code> doesn't catch this target.</p>(¥
cc_static_library≥

name(

deps

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"ÊProduces a static library from a list of targets and their transitive dependencies.

<p>The resulting static library contains the object files of the targets listed in
<code>deps</code> as well as their transitive dependencies, with preference given to
<code>PIC</code> objects.</p>

<h4 id="cc_static_library_output_groups">Output groups</h4>

<h5><code>linkdeps</code></h5>
<p>A text file containing the labels of those transitive dependencies of targets listed in
<code>deps</code> that did not contribute any object files to the static library, but do
provide at least one static, dynamic or interface library. The resulting static library
may require these libraries to be available at link time.</p>

<h5><code>linkopts</code></h5>
<p>A text file containing the user-provided <code>linkopts</code> of all transitive
dependencies of targets listed in <code>deps</code>.

<h4 id="cc_static_library_symbol_check">Duplicate symbols</h4>
<p>By default, the <code>cc_static_library</code> rule checks that the resulting static
library does not contain any duplicate symbols. If it does, the build fails with an error
message that lists the duplicate symbols and the object files containing them.</p>

<p>This check can be disabled per target or per package by setting
<code>features = ["-symbol_check"]</code> or globally via
<code>--features=-symbol_check</code>.</p>

<h5 id="cc_static_library_symbol_check_toolchain">Toolchain support for <code>symbol_check</code></h5>
<p>The auto-configured C++ toolchains shipped with Bazel support the
<code>symbol_check</code> feature on all platforms. Custom toolchains can add support for
it in one of two ways:</p>
<ul>
  <li>Implementing the <code>ACTION_NAMES.validate_static_library</code> action and
  enabling it with the <code>symbol_check</code> feature. The tool set in the action is
  invoked with two arguments, the static library to check for duplicate symbols and the
  path of a file that must be created if the check passes.</li>
  <li>Having the <code>symbol_check</code> feature add archiver flags that cause the
  action creating the static library to fail on duplicate symbols.</li>
</ul>(¡
cc_test¶

name(

deps

srcs

data

additional_linker_inputs

args

aspect_hints

compatible_with

	conlyopts

copts
	
cxxopts
	
defines

deprecation


distribs

dynamic_deps

env

env_inherit

exec_compatible_with

exec_group_compatible_with

exec_properties


features

flaky


hdrs_check


includes


licenses

link_extra_lib


linkopts


linkshared


linkstatic

local

local_defines

malloc

module_interfaces
	
nocopts

package_metadata

reexport_deps

restricted_to

shard_count

size

stamp

tags

target_compatible_with


testonly
	
timeout


toolchains


visibility

win_def_file

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies

$bzl_load_label"ä
<p>
A <code>cc_test()</code> rule compiles a test.  Here, a test
is a binary wrapper around some testing code.
</p>

<p><i>By default, C++ tests are dynamically linked.</i><br/>
    To statically link a unit test, specify
    <a href="#cc_binary.linkstatic"><code>linkstatic=True</code></a>.
    It would probably be good to comment why your test needs
    <code>linkstatic</code>; this is probably not obvious.</p>

<h4>Implicit output targets</h4>
<ul>
<li><code><var>name</var>.stripped</code> (only built if explicitly requested): A stripped
  version of the binary. <code>strip -g</code> is run on the binary to remove debug
  symbols.  Additional strip options can be provided on the command line using
  <code>--stripopt=-foo</code>.</li>
<li><code><var>name</var>.dwp</code> (only built if explicitly requested): If
  <a href="https://gcc.gnu.org/wiki/DebugFission">Fission</a> is enabled: a debug
  information package file suitable for debugging remotely deployed binaries. Else: an
  empty file.</li>
</ul>

<p>
See the <a href="#cc_binary_args">cc_binary()</a> arguments, except that
the <code>stamp</code> argument is set to 0 by default for tests and
that <code>cc_test</code> has extra <a href="#common-attributes-tests">
attributes common to all test rules (*_test)</a>.</p>(Œ
cc_toolchainπ

name(

	all_files(


ar_files


as_files

aspect_hints

compatible_with

compiler_files(
!
compiler_files_without_includes

coverage_files

deprecation

	dwp_files(

dynamic_runtime_lib

exec_compatible_with

exec_group_compatible_with

exec_properties

exec_transition_for_inputs


features


libc_top


licenses

linker_files(


module_map

objcopy_files(

output_licenses

package_metadata

restricted_to

static_runtime_lib

strip_files(

supports_header_parsing

supports_param_files

tags

target_compatible_with


testonly

toolchain_config(

toolchain_identifier


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"ˇ<p>Represents a C++ toolchain.</p>

<p>
  This rule is responsible for:

  <ul>
    <li>
      Collecting all artifacts needed for C++ actions to run. This is done by
      attributes such as <code>all_files</code>, <code>compiler_files</code>,
      <code>linker_files</code>, or other attributes ending with <code>_files</code>). These are
      most commonly filegroups globbing all required files.
    </li>
    <li>
      Generating correct command lines for C++ actions. This is done using
      <code>CcToolchainConfigInfo</code> provider (details below).
    </li>
  </ul>
</p>
<p>
  Use <code>toolchain_config</code> attribute to configure the C++ toolchain.
  See also this
  <a href="https://bazel.build/docs/cc-toolchain-config-reference">
    page
  </a> for elaborate C++ toolchain configuration and toolchain selection documentation.
</p>
<p>
  Use <code>tags = ["manual"]</code> in order to prevent toolchains from being built and configured
  unnecessarily when invoking <code>bazel build //...</code>
</p>(∑
fdo_prefetch_hints∏

name(

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

profile(

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"„<p>Represents an FDO prefetch hints profile that is either in the workspace.
Examples:</p>

<pre><code class="lang-starlark">
fdo_prefetch_hints(
    name = "hints",
    profile = "//path/to/hints:profile.afdo",
)
</code></pre>(±
fdo_profile‹

name(

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

memprof_profile

package_metadata

profile(

proto_profile

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"¿<p>Represents an FDO profile that is in the workspace.
Example:</p>

<pre><code class="lang-starlark">
fdo_profile(
    name = "fdo",
    profile = "//path/to/fdo:profile.zip",
)
</code></pre>(¢
memprof_profile∏

name(

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

profile(

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"—<p>Represents a MEMPROF profile that is in the workspace.
Example:</p>

<pre><code class="lang-starlark">
memprof_profile(
    name = "memprof",
    profile = "//path/to/memprof:profile.afdo",
)

</code></pre>(„
propeller_optimizeÀ

name(

aspect_hints


cc_profile(

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


ld_profile(

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"¸<p>Represents a Propeller optimization profile in the workspace.
Example:</p>

<pre><code class="lang-starlark">
propeller_optimize(
    name = "layout",
    cc_profile = "//path:cc_profile.txt",
    ld_profile = "//path:ld_profile.txt"
)
</code></pre>(˛$
java_binaryí

name(

deps

srcs

data

	resources

add_exports

	add_opens

args

aspect_hints

bootclasspath

classpath_resources

compatible_with

create_executable


deploy_env

deploy_manifest_lines

deprecation

env

exec_compatible_with

exec_group_compatible_with

exec_properties


features

	javacopts

	jvm_flags


launcher


licenses


main_class

	neverlink

output_licenses

package_metadata
	
plugins

resource_strip_prefix

restricted_to

runtime_deps

stamp

tags

target_compatible_with


testonly


toolchains

use_launcher

use_testrunner


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"◊<p>
  Builds a Java archive ("jar file"), plus a wrapper shell script with the same name as the rule.
  The wrapper shell script uses a classpath that includes, among other things, a jar file for each
  library on which the binary depends. When running the wrapper shell script, any nonempty
  <code>JAVABIN</code> environment variable will take precedence over the version specified via
  Bazel's <code>--java_runtime_version</code> flag.
</p>
<p>
  The wrapper script accepts several unique flags. Refer to
  <code>java_stub_template.txt</code>
  for a list of configurable flags and environment variables accepted by the wrapper.
</p>

<h4 id="java_binary_implicit_outputs">Implicit output targets</h4>
<ul>
  <li><code><var>name</var>.jar</code>: A Java archive, containing the class files and other
    resources corresponding to the binary's direct dependencies.</li>
  <li><code><var>name</var>-src.jar</code>: An archive containing the sources ("source
    jar").</li>
  <li><code><var>name</var>_deploy.jar</code>: A Java archive suitable for deployment (only
    built if explicitly requested).
    <p>
      Building the <code>&lt;<var>name</var>&gt;_deploy.jar</code> target for your rule
      creates a self-contained jar file with a manifest that allows it to be run with the
      <code>java -jar</code> command or with the wrapper script's <code>--singlejar</code>
      option. Using the wrapper script is preferred to <code>java -jar</code> because it
      also passes the <a href="#java_binary-jvm_flags">JVM flags</a> and the options
      to load native libraries.
    </p>
    <p>
      The deploy jar contains all the classes that would be found by a classloader that
      searched the classpath from the binary's wrapper script from beginning to end. It also
      contains the native libraries needed for dependencies. These are automatically loaded
      into the JVM at runtime.
    </p>
    <p>If your target specifies a <a href="#java_binary.launcher">launcher</a>
      attribute, then instead of being a normal JAR file, the _deploy.jar will be a
      native binary. This will contain the launcher plus any native (C++) dependencies of
      your rule, all linked into a static binary. The actual jar file's bytes will be
      appended to that native binary, creating a single binary blob containing both the
      executable and the Java code. You can execute the resulting jar file directly
      like you would execute any native binary.</p>
  </li>
  <li><code><var>name</var>_deploy-src.jar</code>: An archive containing the sources
    collected from the transitive closure of the target. These will match the classes in the
    <code>deploy.jar</code> except where jars have no matching source jar.</li>
</ul>

<p>
It is good practice to use the name of the source file that is the main entry point of the
application (minus the extension). For example, if your entry point is called
<code>Main.java</code>, then your name could be <code>Main</code>.
</p>

<p>
  A <code>deps</code> attribute is not allowed in a <code>java_binary</code> rule without
  <a href="#java_binary-srcs"><code>srcs</code></a>; such a rule requires a
  <a href="#java_binary-main_class"><code>main_class</code></a> provided by
  <a href="#java_binary-runtime_deps"><code>runtime_deps</code></a>.
</p>

<p>The following code snippet illustrates a common mistake:</p>

<pre class="code">
<code class="lang-starlark">
java_binary(
    name = "DontDoThis",
    srcs = [
        <var>...</var>,
        <code class="deprecated">"GeneratedJavaFile.java"</code>,  # a generated .java file
    ],
    deps = [<code class="deprecated">":generating_rule",</code>],  # rule that generates that file
)
</code>
</pre>

<p>Do this instead:</p>

<pre class="code">
<code class="lang-starlark">
java_binary(
    name = "DoThisInstead",
    srcs = [
        <var>...</var>,
        ":generating_rule",
    ],
)
</code>
</pre>(Â
java_import¥

name(

deps

data

add_exports

	add_opens

aspect_hints

compatible_with

constraints

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties
	
exports


features

jars(


licenses

	neverlink

package_metadata

proguard_specs

restricted_to

runtime_deps

srcjar

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"ú<p>
  This rule allows the use of precompiled <code>.jar</code> files as
  libraries for <code><a href="#java_library">java_library</a></code> and
  <code>java_binary</code> rules.
</p>

<h4 id="java_import_examples">Examples</h4>

<pre class="code">
<code class="lang-starlark">
    java_import(
        name = "maven_model",
        jars = [
            "maven_model/maven-aether-provider-3.2.3.jar",
            "maven_model/maven-model-3.2.3.jar",
            "maven_model/maven-model-builder-3.2.3.jar",
        ],
    )
</code>
</pre>(‡
java_libraryï

name(

deps

srcs

data

	resources

add_exports

	add_opens

aspect_hints

bootclasspath

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties

exported_plugins
	
exports


features

javabuilder_jvm_flags

	javacopts


licenses

	neverlink

package_metadata
	
plugins

proguard_specs

resource_strip_prefix

restricted_to

runtime_deps

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"µ<p>This rule compiles and links sources into a <code>.jar</code> file.</p>

<h4>Implicit outputs</h4>
<ul>
  <li><code>lib<var>name</var>.jar</code>: A Java archive containing the class files.</li>
  <li><code>lib<var>name</var>-src.jar</code>: An archive containing the sources ("source
    jar").</li>
</ul>(≥
	java_test¬

name(

deps

srcs

data

	resources

add_exports

	add_opens

args

aspect_hints

bootclasspath

classpath_resources

compatible_with

create_executable

deploy_manifest_lines

deprecation

env

env_inherit

exec_compatible_with

exec_group_compatible_with

exec_properties


features

flaky

	javacopts

	jvm_flags


launcher


licenses

local


main_class

	neverlink

package_metadata
	
plugins

resource_strip_prefix

restricted_to

runtime_deps

shard_count

size

stamp

tags

target_compatible_with


test_class


testonly
	
timeout


toolchains

use_launcher

use_testrunner


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"ﬁ	<p>
A <code>java_test()</code> rule compiles a Java test. A test is a binary wrapper around your
test code. The test runner's main method is invoked instead of the main class being compiled.
</p>

<h4 id="java_test_implicit_outputs">Implicit output targets</h4>
<ul>
  <li><code><var>name</var>.jar</code>: A Java archive.</li>
  <li><code><var>name</var>_deploy.jar</code>: A Java archive suitable
    for deployment. (Only built if explicitly requested.) See the description of the
    <code><var>name</var>_deploy.jar</code> output from
    <a href="#java_binary">java_binary</a> for more details.</li>
</ul>

<p>
See the section on <code>java_binary()</code> arguments. This rule also
supports all <a href="https://bazel.build/reference/be/common-definitions#common-attributes-tests">attributes common
to all test rules (*_test)</a>.
</p>

<h4 id="java_test_examples">Examples</h4>

<pre class="code">
<code class="lang-starlark">

java_library(
    name = "tests",
    srcs = glob(["*.java"]),
    deps = [
        "//java/com/foo/base:testResources",
        "//java/com/foo/testing/util",
    ],
)

java_test(
    name = "AllTests",
    size = "small",
    runtime_deps = [
        ":tests",
        "//util/mysql",
    ],
)
</code>
</pre>(ì	
java_package_configurationÈ

name(

data

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

	javacopts

output_licenses

package_metadata


packages

restricted_to

system

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"Ü<p>
Configuration to apply to a set of packages.
Configurations can be added to
<code><a href="#java_toolchain.javacopts">java_toolchain.javacopts</a></code>s.
</p>

<h4 id="java_package_configuration_example">Example:</h4>

<pre class="code">
<code class="lang-starlark">

java_package_configuration(
    name = "my_configuration",
    packages = [":my_packages"],
    javacopts = ["-Werror"],
)

package_group(
    name = "my_packages",
    packages = [
        "//com/my/project/...",
        "-//com/my/project/testing/...",
    ],
)

java_toolchain(
    ...,
    package_configuration = [
        ":my_configuration",
    ]
)

</code>
</pre>(ë
java_pluginù

name(

deps

srcs

data

	resources

add_exports

	add_opens

aspect_hints

bootclasspath

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

generates_api

javabuilder_jvm_flags

	javacopts


licenses

	neverlink

output_licenses

package_metadata
	
plugins

processor_class

proguard_specs

resource_strip_prefix

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"ﬂ<p>
  <code>java_plugin</code> defines plugins for the Java compiler run by Bazel. The
  only supported kind of plugins are annotation processors. A <code>java_library</code> or
  <code>java_binary</code> rule can run plugins by depending on them via the <code>plugins</code>
  attribute. A <code>java_library</code> can also automatically export plugins to libraries that
  directly depend on it using
  <code><a href="#java_library-exported_plugins">exported_plugins</a></code>.
</p>

<h4 id="java_plugin_implicit_outputs">Implicit output targets</h4>
    <ul>
      <li><code><var>libname</var>.jar</code>: A Java archive.</li>
    </ul>

<p>Arguments are a subset of (and with identical semantics to) those of
<a href="#java_library">java_library()</a>,
except for the addition of the <code>processor_class</code> and
<code>generates_api</code> arguments.</p>(Á
java_runtimeª

name(

srcs

aspect_hints

compatible_with

default_cds

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

hermetic_srcs

hermetic_static_libs

java

	java_home


lib_ct_sym

lib_modules

output_licenses

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains
	
version


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"ñ<p>
Specifies the configuration for a Java runtime.
</p>

<h4 id="java_runtime_example">Example:</h4>

<pre class="code">
<code class="lang-starlark">

java_runtime(
    name = "jdk-9-ea+153",
    srcs = glob(["jdk9-ea+153/**"]),
    java_home = "jdk9-ea+153",
)

</code>
</pre>(Í
java_single_jarˇ

name(

deps

aspect_hints

compatible_with


compress


deploy_env

deploy_manifest_lines

deprecation

exclude_build_data

exclude_pattern

exec_compatible_with

exec_group_compatible_with

exec_properties


features

multi_release

output

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"“Collects Java dependencies and jar files into a single jar

`java_single_jar` collects Java dependencies and jar files into a single jar.
This is similar to java_binary with everything related to executables disabled,
and provides an alternative to the java_binary "deploy jar hack".

## Example

```skylark
load("//tools/build_defs/java_single_jar:java_single_jar.bzl", "java_single_jar")

java_single_jar(
    name = "my_single_jar",
    deps = [
        "//java/com/google/foo",
        "//java/com/google/bar",
    ],
)
```

Outputs:
  {name}.jar: A single jar containing all of the inputs.(’
java_toolchainø

name(

android_lint_data

android_lint_jvm_opts

android_lint_opts
$
"android_lint_package_configuration

android_lint_runner

aspect_hints

bootclasspath

compatible_javacopts

compatible_with

deprecation

deps_checker

exec_compatible_with

exec_group_compatible_with

exec_properties


features
%
#forcibly_disable_header_compilation


genclass

header_compiler
$
"header_compiler_builtin_processors

header_compiler_direct

ijar

jacocorunner

java_runtime

javabuilder

javabuilder_data

javabuilder_jvm_opts
"
 javac_supports_multiplex_workers
$
"javac_supports_worker_cancellation
,
*javac_supports_worker_multiplex_sandboxing

javac_supports_workers

	javacopts

jspecify_implicit_deps

jspecify_javacopts

jspecify_packages

jspecify_processor

jspecify_processor_class

jspecify_stubs


jvm_opts


licenses

misc


oneversion

oneversion_allowlist
 
oneversion_allowlist_for_tests

oneversion_whitelist

package_configuration

package_metadata

proguard_allowlister
+
)reduced_classpath_incompatible_processors

restricted_to

	singlejar

source_version

tags

target_compatible_with

target_version


testonly

timezone_data


toolchains

tools

turbine_data

turbine_jvm_opts


visibility

xlint

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"˛<p>
Specifies the configuration for the Java compiler. Which toolchain to be used can be changed through
the --java_toolchain argument. Normally you should not write those kind of rules unless you want to
tune your Java compiler.
</p>

<h4>Examples</h4>

<p>A simple example would be:
</p>

<pre class="code">
<code class="lang-starlark">

java_toolchain(
    name = "toolchain",
    source_version = "7",
    target_version = "7",
    bootclasspath = ["//tools/jdk:bootclasspath"],
    xlint = [ "classfile", "divzero", "empty", "options", "path" ],
    javacopts = [ "-g" ],
    javabuilder = ":JavaBuilder_deploy.jar",
)
</code>
</pre>(®
objc_import∫

name(

deps

hdrs


alwayslink

archives(

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


includes

package_metadata

restricted_to


sdk_dylibs

sdk_frameworks

sdk_includes

tags

target_compatible_with


testonly

textual_hdrs


toolchains


visibility

weak_sdk_frameworks

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"Ÿ<p>This rule encapsulates an already-compiled static library in the form of an
<code>.a</code> file. It also allows exporting headers and resources using the same
attributes supported by <code>objc_library</code>.</p>(¬
objc_library⁄

name(

deps

srcs

data

hdrs


alwayslink

aspect_hints

compatible_with

	conlyopts

copts
	
cxxopts
	
defines

deprecation

enable_modules

exec_compatible_with

exec_group_compatible_with

exec_properties


features

implementation_deps


includes


linkopts


module_map

module_name

non_arc_srcs

package_metadata

pch

restricted_to


sdk_dylibs

sdk_frameworks

sdk_includes

stamp

tags

target_compatible_with


testonly

textual_hdrs


toolchains


visibility

weak_sdk_frameworks

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"S<p>This rule produces a static library from the given Objective-C source files.</p>(Ú
cc_proto_libraryà

name(

deps

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"–<p>
<code>cc_proto_library</code> generates C++ code from <code>.proto</code> files.
</p>

<p>
<code>deps</code> must point to <a href="protocol-buffer.html#proto_library"><code>proto_library
</code></a> rules.
</p>

<p>
Example:
</p>

<pre>
<code class="lang-starlark">
cc_library(
    name = "lib",
    deps = [":foo_cc_proto"],
)

cc_proto_library(
    name = "foo_cc_proto",
    deps = [":foo_proto"],
)

proto_library(
    name = "foo_proto",
)
</code>
</pre>(Å
java_lite_proto_libraryà

name(

deps

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"ÿ<p>
<code>java_lite_proto_library</code> generates Java code from <code>.proto</code> files.
</p>

<p>
<code>deps</code> must point to <a href="protocol-buffer.html#proto_library"><code>proto_library
</code></a> rules.
</p>

<p>
Example:
</p>

<pre class="code">
<code class="lang-starlark">
java_library(
    name = "lib",
    runtime_deps = [":foo"],
)

java_lite_proto_library(
    name = "foo",
    deps = [":bar"],
)

proto_library(
    name = "bar",
)
</code>
</pre>(†
java_proto_libraryî

name(

deps

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


licenses

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"<p>
<code>java_proto_library</code> generates Java code from <code>.proto</code> files.
</p>

<p>
<code>deps</code> must point to <a href="protocol-buffer.html#proto_library"><code>proto_library
</code></a> rules.
</p>

<p>
Example:
</p>

<pre class="code">
<code class="lang-starlark">
java_library(
    name = "lib",
    runtime_deps = [":foo_java_proto"],
)

java_proto_library(
    name = "foo_java_proto",
    deps = [":foo_proto"],
)

proto_library(
    name = "foo_proto",
)
</code>
</pre>(·
proto_libraryë

name(

deps

srcs

data

allow_exports

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties
	
exports

extension_declarations


features

import_prefix


licenses

option_deps

package_metadata

restricted_to

strip_import_prefix

tags

target_compatible_with


testonly


toolchains


visibility"π<p>Use <code>proto_library</code> to define libraries of protocol buffers which
may be used from multiple languages. A <code>proto_library</code> may be listed
in the <code>deps</code> clause of supported rules, such as
<code>java_proto_library</code>.

<p>When compiled on the command-line, a <code>proto_library</code> creates a file
named <code>foo-descriptor-set.proto.bin</code>, which is the descriptor set for
the messages the rule srcs. The file is a serialized
<code>FileDescriptorSet</code>, which is described in
<a href="https://developers.google.com/protocol-buffers/docs/techniques#self-description">
https://developers.google.com/protocol-buffers/docs/techniques#self-description</a>.

<p>It only contains information about the <code>.proto</code> files directly
mentioned by a <code>proto_library</code> rule; the collection of transitive
descriptor sets is available through the
<code>[ProtoInfo].transitive_descriptor_sets</code> Starlark provider.
See documentation in <code>proto_info.bzl</code>.

<p>Recommended code organization:
<ul>
<li>One <code>proto_library</code> rule per <code>.proto</code> file.
<li>A file named <code>foo.proto</code> will be in a rule named <code>foo_proto</code>,
  which is located in the same package.
<li>A <code>[language]_proto_library</code> that wraps a <code>proto_library</code>
  named <code>foo_proto</code> should be called <code>foo_[language]_proto</code>,
  and be located in the same package.
</ul>(ï
py_proto_libraryà

name(

deps

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"Û      Use `py_proto_library` to generate Python libraries from `.proto` files.

      The convention is to name the `py_proto_library` rule `foo_py_pb2`,
      when it is wrapping `proto_library` rule `foo_proto`.

      `deps` must point to a `proto_library` rule.

      Example:

```starlark
py_library(
    name = "lib",
    deps = [":foo_py_pb2"],
)

py_proto_library(
    name = "foo_py_pb2",
    deps = [":foo_proto"],
)

proto_library(
    name = "foo_proto",
    srcs = ["foo.proto"],
)
```(Ê
proto_lang_toolchainÊ

name(

allowlist_different_package

aspect_hints

blacklisted_protos

command_line(

compatible_with

denylisted_protos

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


mnemonic

output_files

package_metadata

plugin

plugin_format_flag

progress_message

protoc_minimal_do_not_use

restricted_to
	
runtime

tags

target_compatible_with


testonly

toolchain_type


toolchains


visibility"‚<p>If using Bazel, please load the rule from <a href="https://github.com/bazelbuild/rules_proto">
https://github.com/bazelbuild/rules_proto</a>.

<p>Specifies how a LANG_proto_library rule (e.g., <code>java_proto_library</code>) should invoke the
proto-compiler.
Some LANG_proto_library rules allow specifying which toolchain to use using command-line flags;
consult their documentation.

<p>Normally you should not write those kind of rules unless you want to
tune your Java compiler.

<p>There's no compiler. The proto-compiler is taken from the proto_library rule we attach to. It is
passed as a command-line flag to Blaze.
Several features require a proto-compiler to be invoked on the proto_library rule itself.
It's beneficial to enforce the compiler that LANG_proto_library uses is the same as the one
<code>proto_library</code> does.

<h4>Examples</h4>

<p>A simple example would be:
<pre><code class="lang-starlark">
proto_lang_toolchain(
    name = "javalite_toolchain",
    command_line = "--javalite_out=shared,immutable:$(OUT)",
    plugin = ":javalite_plugin",
    runtime = ":protobuf_lite",
)
</code></pre>(Ë
proto_toolchain“

name(

aspect_hints

command_line

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


mnemonic

output_files

package_metadata

progress_message

proto_compiler

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility(Ì
	py_binary›

name(

deps

srcs

data

args

aspect_hints

compatible_with

deprecation


distribs

env

exec_compatible_with

exec_group_compatible_with

exec_properties


features
	
imports

interpreter_args

legacy_create_init


licenses

main

main_module

output_licenses

package_metadata


precompile

precompile_invalidation_mode

precompile_optimize_level

precompile_source_retention

pyc_collection


pyi_deps


pyi_srcs

python_version

restricted_to

srcs_version

stamp

tags

target_compatible_with


testonly


toolchains


visibility(ó

py_libraryÒ

name(

deps

srcs

data

aspect_hints

compatible_with

deprecation


distribs

exec_compatible_with

exec_group_compatible_with

exec_properties
"
 experimental_venvs_site_packages


features
	
imports


licenses

package_metadata


precompile

precompile_invalidation_mode

precompile_optimize_level

precompile_source_retention


pyi_deps


pyi_srcs

restricted_to

srcs_version

tags

target_compatible_with


testonly


toolchains


visibility"íA library of Python code that can be depended upon.

Default outputs:
* The input Python sources
* The precompiled artifacts from the sources.

NOTE: Precompilation affects which of the default outputs are included in the
resulting runfiles. See the precompile-related attributes and flags for
more information.

:::{versionchanged} 0.37.0
Source files are no longer added to the runfiles directly.
:::(õ
py_testç

name(

deps

srcs

data

args

aspect_hints

compatible_with

deprecation


distribs

env

env_inherit

exec_compatible_with

exec_group_compatible_with

exec_properties


features

flaky
	
imports

interpreter_args

legacy_create_init


licenses

local

main

main_module

package_metadata


precompile

precompile_invalidation_mode

precompile_optimize_level

precompile_source_retention

pyc_collection


pyi_deps


pyi_srcs

python_version

restricted_to

shard_count

size

srcs_version

stamp

tags

target_compatible_with


testonly
	
timeout


toolchains


visibility(ª

py_runtime§

name(

	abi_flags

aspect_hints

bootstrap_template

compatible_with

coverage_tool

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

files

implementation_name

interpreter

interpreter_path

interpreter_version_info

package_metadata
	
pyc_tag

python_version

restricted_to

site_init_template

stage2_bootstrap_template

stub_shebang

supports_build_time_venv

tags

target_compatible_with


testonly


toolchains


visibility

zip_main_template"É	Represents a Python runtime used to execute Python code.

A `py_runtime` target can represent either a *platform runtime* or an *in-build
runtime*. A platform runtime accesses a system-installed interpreter at a known
path, whereas an in-build runtime points to an executable target that acts as
the interpreter. In both cases, an "interpreter" means any executable binary or
wrapper script that is capable of running a Python script passed on the command
line, following the same conventions as the standard CPython interpreter.

A platform runtime is by its nature non-hermetic. It imposes a requirement on
the target platform to have an interpreter located at a specific path. An
in-build runtime may or may not be hermetic, depending on whether it points to
a checked-in interpreter or a wrapper script that accesses the system
interpreter.

Example

```
load("@rules_python//python:py_runtime.bzl", "py_runtime")

py_runtime(
    name = "python-2.7.12",
    files = glob(["python-2.7.12/**"]),
    interpreter = "python-2.7.12/bin/python",
)

py_runtime(
    name = "python-3.6.0",
    interpreter_path = "/opt/pyenv/versions/3.6.0/bin/python",
)
```(Ø	
	sh_binaryﬁ

name(

deps

srcs

data

args

aspect_hints

compatible_with

deprecation

env

env_inherit

exec_compatible_with

exec_group_compatible_with

exec_properties


features

output_licenses

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains

use_bash_launcher


visibility"æ<p>
  The <code>sh_binary</code> rule is used to declare executable shell scripts.
  (<code>sh_binary</code> is a misnomer: its outputs aren't necessarily binaries.) This rule ensures
  that all dependencies are built, and appear in the <code>runfiles</code> area at execution time.
  We recommend that you name your <code>sh_binary()</code> rules after the name of the script minus
  the extension (e.g. <code>.sh</code>); the rule name and the file name must be distinct.
  <code>sh_binary</code> respects shebangs, so any available interpreter may be used (eg.
  <code>#!/bin/zsh</code>)
</p>
<h4 id="sh_binary_examples">Example</h4>
<p>For a simple shell script with no dependencies and some data files:
</p>
<pre class="code">
sh_binary(
    name = "foo",
    srcs = ["foo.sh"],
    data = glob(["datafiles/*.txt"]),
)
</pre>(Ò

sh_libraryò

name(

deps

srcs

data

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"≈
<p>
  The main use for this rule is to aggregate together a logical
  "library" consisting of related scripts&mdash;programs in an
  interpreted language that does not require compilation or linking,
  such as the Bourne shell&mdash;and any data those programs need at
  run-time. Such "libraries" can then be used from
  the <code>data</code> attribute of one or
  more <code>sh_binary</code> rules.
</p>

<p>
  You can use the <a href="#filegroup"><code>filegroup</code></a> rule to aggregate data
  files.
</p>

<p>
  In interpreted programming languages, there's not always a clear
  distinction between "code" and "data": after all, the program is
  just "data" from the interpreter's point of view. For this reason
  this rule has three attributes which are all essentially equivalent:
  <code>srcs</code>, <code>deps</code> and <code>data</code>.
  The current implementation does not distinguish between the elements of these lists.
  All three attributes accept rules, source files and generated files.
  It is however good practice to use the attributes for their usual purpose (as with other rules).
</p>

<h4 id="sh_library_examples">Examples</h4>

<pre class="code">
sh_library(
    name = "foo",
    data = [
        ":foo_service_script",  # an sh_binary with srcs
        ":deploy_foo",  # another sh_binary with srcs
    ],
)
</pre>(∫
sh_testˇ

name(

deps

srcs

data

args

aspect_hints

compatible_with

deprecation

env

env_inherit

exec_compatible_with

exec_group_compatible_with

exec_properties


features

flaky

local

package_metadata

restricted_to

shard_count

size

tags

target_compatible_with


testonly
	
timeout


toolchains

use_bash_launcher


visibility"™<p>A <code>sh_test()</code> rule creates a test written as a Bourne shell script.</p>

<p>See the <a href="#common-attributes-tests">
attributes common to all test rules (*_test)</a>.</p>

<h4 id="sh_test_examples">Examples</h4>

<pre class="code">
sh_test(
    name = "foo_integration_test",
    size = "small",
    srcs = ["foo_integration_test.sh"],
    deps = [":foo_sh_lib"],
    data = glob(["testdata/*.txt"]),
)
</pre>(ü
action_listenerò

name(

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties

extra_actions(


features


licenses

	mnemonics(

package_metadata

restricted_to

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs"Ó

<p>
  <b>WARNING:</b> Extra actions are deprecated. Use
  <a href="https://bazel.build/rules/aspects">aspects</a>
  instead.
</p>

<p>
  An <code>action_listener</code> rule doesn't produce any output itself.
  Instead, it allows tool developers to insert
  <a href="#extra_action"><code>extra_action</code></a>s into the build system,
  by providing a mapping from action to <a href="#extra_action"><code>extra_action</code>
  </a>.
</p>

<p>
  This rule's arguments map action mnemonics to
  <a href="#extra_action"><code>extra_action</code></a> rules.
</p>

<p>
  By specifying the option <a href="/docs/user-manual#flag--experimental_action_listener">
  <code>--experimental_action_listener=&lt;label&gt;</code></a>,
  the build will use the specified <code>action_listener</code> to insert
  <a href="#extra_action"><code>extra_action</code></a>s into the build graph.
</p>

<h4 id="action_listener_example">Example</h4>
<pre>
action_listener(
    name = "index_all_languages",
    mnemonics = [
        "Javac",
        "CppCompile",
        "Python",
    ],
    extra_actions = [":indexer"],
)

action_listener(
    name = "index_java",
    mnemonics = ["Javac"],
    extra_actions = [":indexer"],
)

extra_action(
    name = "indexer",
    tools = ["//my/tools:indexer"],
    cmd = "$(location //my/tools:indexer)" +
          "--extra_action_file=$(EXTRA_ACTION_FILE)",
)
</pre>

(†
extra_action…

name(

data

aspect_hints

cmd(

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


licenses

out_templates

package_metadata

requires_action_output

restricted_to

tags

target_compatible_with


testonly


toolchains

tools


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs"¡
<p>
  <b>WARNING:</b> Extra actions are deprecated. Use
  <a href="https://bazel.build/rules/aspects">aspects</a>
  instead.
</p>

<p>
  An <code>extra_action</code> rule doesn't produce any meaningful output
  when specified as a regular build target. Instead, it allows tool developers
  to insert additional actions into the build graph that shadow existing actions.
</p>

<p>
  See <a href="#action_listener"><code>action_listener</code></a> for details
  on how to enable <code>extra_action</code>s.
</p>

<p>
  The <code>extra_action</code>s run as a command-line. The command-line tool gets
  access to a file containing a protocol buffer as $(EXTRA_ACTION_FILE)
  with detailed information on the original action it is shadowing.
  It also has access to all the input files the original action has access to.
  See <tt>extra_actions_base.proto</tt>
  for details on the data stored inside the protocol buffer. Each proto file
  contains an ExtraActionInfo message.
</p>

<p>
  Just like all other actions, extra actions are sandboxed, and should be designed to handle that.
</p>

(ê
alias°

name(


actual(

aspect_hints

compatible_with

deprecation


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies"‡
<p>
  The <code>alias</code> rule creates another name a rule can be referred to as.
</p>

<p>
  Aliasing only works for "regular" targets. In particular, <code>package_group</code>
  and <code>test_suite</code> cannot be aliased.
</p>

<p>
  Aliasing may be of help in large repositories where renaming a target would require making
  changes to lots of files. You can also use alias rule to store a
  <a href="#select">select</a> function call if you want to reuse that logic for
  multiple targets.
</p>

<p>
  The alias rule has its own visibility declaration. In all other respects, it behaves
  like the rule it references (e.g. testonly <em>on the alias</em> is ignored; the testonly-ness
   of the referenced rule is used instead) with some minor exceptions:

  <ul>
    <li>
      Tests are not run if their alias is mentioned on the command line. To define an alias
      that runs the referenced test, use a <a href="#test_suite"><code>test_suite</code></a>
      rule with a single target in its <a href="#test_suite.tests"><code>tests</code></a>
      attribute.
    </li>
    <li>
      When defining environment groups, the aliases to <code>environment</code> rules are not
      supported. They are not supported in the <code>--target_environment</code> command line
      option, either.
    </li>
  </ul>
</p>

<h4 id="alias_example">Examples</h4>

<pre class="code">
filegroup(
    name = "data",
    srcs = ["data.txt"],
)

alias(
    name = "other",
    actual = ":data",
)
</pre>

(…*
config_setting⁄

name(

aspect_hints

constraint_values

define_values

deprecation


features

flag_values


licenses

package_metadata

tags


testonly

values


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

:flag_alias_settings"◊'
  <p>
    Matches an expected configuration state (expressed as build flags or platform constraints) for
    the purpose of triggering configurable attributes. See <a href="#select">select</a> for
    how to consume this rule and <a href="#configurable-attributes">
    Configurable attributes</a> for an overview of the general feature.

  <h4 id="config_setting_examples">Examples</h4>

  <p>The following matches any build that sets <code>--compilation_mode=opt</code> or
  <code>-c opt</code> (either explicitly at the command line or implicitly from .bazelrc files):
  </p>

  <pre class="code">
  config_setting(
      name = "simple",
      values = {"compilation_mode": "opt"}
  )
  </pre>

  <p>The following matches any build that targets ARM and applies the custom define
  <code>FOO=bar</code> (for instance, <code>bazel build --cpu=arm --define FOO=bar ...</code>):
  </p>

  <pre class="code">
  config_setting(
      name = "two_conditions",
      values = {
          "cpu": "arm",
          "define": "FOO=bar"
      }
  )
  </pre>

  <p>The following matches any build that sets
     <a href="https://bazel.build/rules/config#user-defined-build-settings">user-defined flag</a>
     <code>--//custom_flags:foo=1</code> (either explicitly at the command line or implicitly from
     .bazelrc files):
  </p>

  <pre class="code">
  config_setting(
      name = "my_custom_flag_is_set",
      flag_values = { "//custom_flags:foo": "1" },
  )
  </pre>

  <p>The following matches any build that targets a platform with an x86_64 architecture and glibc
     version 2.25, assuming the existence of a <code>constraint_value</code> with label
     <code>//example:glibc_2_25</code>. Note that a platform still matches if it defines additional
     constraint values beyond these two.
  </p>

  <pre class="code">
  config_setting(
      name = "64bit_glibc_2_25",
      constraint_values = [
          "@platforms//cpu:x86_64",
          "//example:glibc_2_25",
      ]
  )
  </pre>

  In all these cases, it's possible for the configuration to change within the build, for example if
  a target needs to be built for a different platform than its dep. This means that even when a
  <code>config_setting</code> doesn't match the top-level command-line flags, it may still match
  some build targets.

  <h4 id="config_setting_notes">Notes</h4>
  <ul>
    <li>See <a href="#select">select</a> for what happens when multiple
       <code>config_setting</code>s match the current configuration state.
    </li>

    <li>For flags that support shorthand forms (e.g. <code>--compilation_mode</code> vs.
      <code>-c</code>), <code>values</code> definitions must use the full form. These automatically
      match invocations using either form.
    </li>

    <li>
      If a flag takes multiple values (like <code>--copt=-Da --copt=-Db</code> or a list-typed
      <a href="https://bazel.build/rules/config#user-defined-build-settings">
      Starlark flag</a>), <code>values = { "flag": "a" }</code> matches if <code>"a"</code> is
      present <i>anywhere</i> in the actual list.

      <p>
        <code>values = { "myflag": "a,b" }</code> works the same way: this matches
        <code>--myflag=a --myflag=b</code>, <code>--myflag=a --myflag=b --myflag=c</code>,
        <code>--myflag=a,b</code>, and <code>--myflag=c,b,a</code>. Exact semantics vary between
        flags. For example, <code>--copt</code> doesn't support multiple values <i>in the same
        instance</i>: <code>--copt=a,b</code> produces <code>["a,b"]</code> while <code>--copt=a
        --copt=b</code> produces <code>["a", "b"]</code> (so <code>values = { "copt": "a,b" }</code>
        matches the former but not the latter). But <code>--ios_multi_cpus</code> (for Apple rules)
        <i>does</i>: <code>-ios_multi_cpus=a,b</code> and <code>ios_multi_cpus=a --ios_multi_cpus=b
        </code> both produce <code>["a", "b"]</code>. Check flag definitions and test your
        conditions carefully to verify exact expectations.
      </p>
    </li>

    <li>If you need to define conditions that aren't modeled by built-in build flags, use
      <a href="https://bazel.build/rules/config#user-defined-build-settings">
      Starlark-defined flags</a>. You can also use <code>--define</code>, but this offers weaker
      support and is not recommended. See
      <a href="#configurable-attributes">here</a> for more discussion.
    </li>

    <li>Avoid repeating identical <code>config_setting</code> definitions in different packages.
      Instead, reference a common <code>config_setting</code> that defined in a canonical package.
    </li>

    <li><a href="general.html#config_setting.values"><code>values</code></a>,
       <a href="general.html#config_setting.define_values"><code>define_values</code></a>, and
       <a href="general.html#config_setting.constraint_values"><code>constraint_values</code></a>
       can be used in any combination in the same <code>config_setting</code> but at least one must
       be set for any given <code>config_setting</code>.
    </li>
  </ul>
(„
	filegroupÙ

name(

srcs

data

aspect_hints

compatible_with

deprecation


features


licenses

output_group

package_metadata

restricted_to

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

output_licenses"‹<p>
  Use <code>filegroup</code> to gather the outputs of a set of targets under a single
  label.
</p>

<p>
  <code>filegroup</code> is not a substitute for listing targets on the command line or
  in an attribute of another rule, because targets have many properties other than their
  outputs, which are not collected in the same way. However, it's still useful in quite
  a few cases, for example, in the <code>srcs</code> attribute of a genrule, or
  the <code>data</code> attribute of a *_binary rule.
</p>

<p>
  Using <code>filegroup</code> is encouraged instead of referencing directories directly.
  Directly referencing directories is discouraged because the build system does not have
  full knowledge of all files below the directory, so it may not rebuild when these files change.
  When combined with <a href="#glob">glob</a>, <code>filegroup</code> can ensure that all
  files are explicitly known to the build system.
</p>

<h4 id="filegroup_example">Examples</h4>

<p>
  To create a <code>filegroup</code> consisting of two source files, do
</p>
<pre class="code">
filegroup(
    name = "mygroup",
    srcs = [
        "a_file.txt",
        "//a/library:target",
        "//a/binary:target",
    ],
)
</pre>
<p>
  Or, use a <code>glob</code> to fully crawl a testdata directory:
</p>
<pre class="code">
filegroup(
    name = "exported_testdata",
    srcs = glob([
        "testdata/*.dat",
        "testdata/logs/**&#47;*.log",
    ]),
)
</pre>
<p>
  To make use of these definitions, reference the <code>filegroup</code> with a label from any rule:
</p>
<pre class="code">
cc_library(
    name = "my_library",
    srcs = ["foo.cc"],
    data = [
        "//my_package:exported_testdata",
        "//my_package:mygroup",
    ],
)
</pre>

(ƒ
genquery‹

name(

deps

data

aspect_hints

compatible_with

compressed_output

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


expression(


features


licenses

opts

package_metadata

restricted_to
	
scope(

strict

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs"÷
  <p>
  <code>genquery()</code> runs a query specified in the
    <a href="/reference/query">Bazel query language</a> and dumps the result
    into a file.
  </p>
  <p>
    In order to keep the build consistent, the query is allowed only to visit
    the transitive closure of the targets specified in the <code>scope</code>
    attribute. Queries violating this rule will fail during execution if
    <code>strict</code> is unspecified or true (if <code>strict</code> is false,
    the out of scope targets will simply be skipped with a warning). The
    easiest way to make sure this does not happen is to mention the same labels
    in the scope as in the query expression.
  </p>
  <p>
    The only difference between the queries allowed here and on the command
    line is that queries containing wildcard target specifications (e.g.
    <code>//pkg:*</code> or <code>//pkg:all</code>) are not allowed here.
    The reasons for this are two-fold: first, because <code>genquery</code> has
    to specify a scope to prevent targets outside the transitive closure of the
    query to influence its output; and, second, because <code>BUILD</code> files
    do not support wildcard dependencies (e.g. <code>deps=["//a/..."]</code>
    is not allowed).
  </p>
  <p>
    The genquery's output is ordered lexicographically in order to enforce deterministic output,
    with the exception of <code>--output=graph|minrank|maxrank</code> or when <code>somepath</code>
    is used as the top-level function.
  <p>
    The name of the output file is the name of the rule.
  </p>

<h4 id="genquery_examples">Examples</h4>
  <p>
    This example writes the list of the labels in the transitive closure of the
    specified target to a file.
  </p>

<pre class="code">
genquery(
    name = "kiwi-deps",
    expression = "deps(//kiwi:kiwi_lib)",
    scope = ["//kiwi:kiwi_lib"],
)
</pre>

(åK
genruleÓ

name(

srcs

outs(

aspect_hints

cmd


cmd_bash
	
cmd_bat

cmd_ps

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


executable


features


licenses

local
	
message

output_licenses

output_to_bindir

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains

tools


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$is_executable

heuristic_label_expansion

$genrule_setup

stamp"çF
<p>A <code>genrule</code> generates one or more files using a user-defined Bash command.</p>

<p>
  Genrules are generic build rules that you can use if there's no specific rule for the task.
  For example, you could run a Bash one-liner. If however you need to compile C++ files, stick
  to the existing <code>cc_*</code> rules, because all the heavy lifting has already been done
  for you.
</p>
<p>
  Note that genrule requires a shell to interpret the command argument.
  It is also easy to reference arbitrary programs available on the PATH, however this makes the
  command non-hermetic and may not be reproducible.
  If you only need to run a single tool, consider using
  <a href="https://github.com/bazelbuild/bazel-skylib/blob/main/docs/run_binary_doc.md">run_binary</a>
  instead.
</p>
<p>
  Like every other action, the action created by genrules should not assume anything about their
  working directory; all Bazel guarantees is that their declared inputs will be available at the
  path that <code>$(location)</code> returns for their label. For example, if the action is run in a
  sandbox or remotely, the implementation of the sandbox or the remote execution will determine the
  working directory. If run directly (using the <code>standalone</code> strategy), the working
  directory will be the execution root, i.e. the result of <code>bazel info execution_root</code>.
</p>
<p>
  Do not use a genrule for running tests. There are special dispensations for tests and test
  results, including caching policies and environment variables. Tests generally need to be run
  after the build is complete and on the target architecture, whereas genrules are executed during
  the build and on the exec architecture (the two may be different). If you need a general purpose
  testing rule, use <a href="#sh_test"><code>sh_test</code></a>.
</p>

<h4>Cross-compilation Considerations</h4>

<p>
  <em>See <a href="/docs/user-manual#configurations">the user manual</a> for more info about
  cross-compilation.</em>
</p>
<p>
  While genrules run during a build, their outputs are often used after the build, for deployment or
  testing. Consider the example of compiling C code for a microcontroller: the compiler accepts C
  source files and generates code that runs on a microcontroller. The generated code obviously
  cannot run on the CPU that was used for building it, but the C compiler (if compiled from source)
  itself has to.
</p>
<p>
  The build system uses the exec configuration to describe the machine(s) on which the build runs
  and the target configuration to describe the machine(s) on which the output of the build is
  supposed to run. It provides options to configure each of these and it segregates the
  corresponding files into separate directories to avoid conflicts.
</p>
<p>
  For genrules, the build system ensures that dependencies are built appropriately:
  <code>srcs</code> are built (if necessary) for the <em>target</em> configuration,
  <code>tools</code> are built for the <em>exec</em> configuration, and the output is considered to
  be for the <em>target</em> configuration. It also provides <a href="#make-variables">
  "Make" variables</a> that genrule commands can pass to the corresponding tools.
</p>
<p>
  It is intentional that genrule defines no <code>deps</code> attribute: other built-in rules use
  language-dependent meta information passed between the rules to automatically determine how to
  handle dependent rules, but this level of automation is not possible for genrules. Genrules work
  purely at the file and runfiles level.
</p>

<h4>Special Cases</h4>

<p>
  <i>Exec-exec compilation</i>: in some cases, the build system needs to run genrules such that the
  output can also be executed during the build. If for example a genrule builds some custom compiler
  which is subsequently used by another genrule, the first one has to produce its output for the
  exec configuration, because that's where the compiler will run in the other genrule. In this case,
  the build system does the right thing automatically: it builds the <code>srcs</code> and
  <code>outs</code> of the first genrule for the exec configuration instead of the target
  configuration. See <a href="/docs/user-manual#configurations">the user manual</a> for more
  info.
</p>
<p>
  <i>JDK & C++ Tooling</i>: to use a tool from the JDK or the C++ compiler suite, the build system
  provides a set of variables to use. See <a href="#make-variables">"Make" variable</a> for
  details.
</p>

<h4>Genrule Environment</h4>

<p>
  The genrule command is executed by a Bash shell that is configured to fail when a command
  or a pipeline fails, using <code>set -e -o pipefail</code>.
</p>
<p>
  The build tool executes the Bash command in a sanitized process environment that
  defines only core variables such as <code>PATH</code>, <code>PWD</code>,
  <code>TMPDIR</code>, and a few others.

  To ensure that builds are reproducible, most variables defined in the user's shell
  environment are not passed though to the genrule's command. However, Bazel (but not
  Blaze) passes through the value of the user's <code>PATH</code> environment variable.

  Any change to the value of <code>PATH</code> will cause Bazel to re-execute the command
  on the next build.
  <!-- See https://github.com/bazelbuild/bazel/issues/1142 -->
</p>
<p>
  A genrule command should not access the network except to connect processes that are
  children of the command itself, though this is not currently enforced.
</p>
<p>
  The build system automatically deletes any existing output files, but creates any necessary parent
  directories before it runs a genrule. It also removes any output files in case of a failure.
</p>

<h4>General Advice</h4>

<ul>
  <li>Do ensure that tools run by a genrule are deterministic and hermetic. They should not write
    timestamps to their output, and they should use stable ordering for sets and maps, as well as
    write only relative file paths to the output, no absolute paths. Not following this rule will
    lead to unexpected build behavior (Bazel not rebuilding a genrule you thought it would) and
    degrade cache performance.</li>
  <li>Do use <code>$(location)</code> extensively, for outputs, tools and sources. Due to the
    segregation of output files for different configurations, genrules cannot rely on hard-coded
    and/or absolute paths.</li>
  <li>Do write a common Starlark macro in case the same or very similar genrules are used in
    multiple places. If the genrule is complex, consider implementing it in a script or as a
    Starlark rule. This improves readability as well as testability.</li>
  <li>Do make sure that the exit code correctly indicates success or failure of the genrule.</li>
  <li>Do not write informational messages to stdout or stderr. While useful for debugging, this can
    easily become noise; a successful genrule should be silent. On the other hand, a failing genrule
    should emit good error messages.</li>
  <li><code>$$</code> evaluates to a <code>$</code>, a literal dollar-sign, so in order to invoke a
    shell command containing dollar-signs such as <code>ls $(dirname $x)</code>, one must escape it
    thus: <code>ls $$(dirname $$x)</code>.</li>
  <li>Avoid creating symlinks and directories. Bazel doesn't copy over the directory/symlink
    structure created by genrules and its dependency checking of directories is unsound.</li>
  <li>When referencing the genrule in other rules, you can use either the genrule's label or the
    labels of individual output files. Sometimes the one approach is more readable, sometimes the
    other: referencing outputs by name in a consuming rule's <code>srcs</code> will avoid
    unintentionally picking up other outputs of the genrule, but can be tedious if the genrule
    produces many outputs.</li>
</ul>

<h4 id="genrule_examples">Examples</h4>

<p>
  This example generates <code>foo.h</code>. There are no sources, because the command doesn't take
  any input. The "binary" run by the command is a perl script in the same package as the genrule.
</p>
<pre class="code">
genrule(
    name = "foo",
    srcs = [],
    outs = ["foo.h"],
    cmd = "./$(location create_foo.pl) &gt; \"$@\"",
    tools = ["create_foo.pl"],
)
</pre>

<p>
  The following example shows how to use a <a href="#filegroup"><code>filegroup</code>
  </a> and the outputs of another <code>genrule</code>. Note that using <code>$(SRCS)</code> instead
  of explicit <code>$(location)</code> directives would also work; this example uses the latter for
  sake of demonstration.
</p>
<pre class="code">
genrule(
    name = "concat_all_files",
    srcs = [
        "//some:files",  # a filegroup with multiple files in it ==> $(location<b>s</b>)
        "//other:gen",   # a genrule with a single output ==> $(location)
    ],
    outs = ["concatenated.txt"],
    cmd = "cat $(locations //some:files) $(location //other:gen) > $@",
)
</pre>

(˘
starlark_doc_extractÈ

name(

deps

src(

data

allow_unused_doc_comments

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


licenses

package_metadata

render_main_repo_name

restricted_to

symbol_names

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs"Ú	
<p><code>starlark_doc_extract()</code> extracts documentation for rules, functions (including
macros), aspects, and providers defined or re-exported in a given <code>.bzl</code> or
<code>.scl</code> file. The output of this rule is a <code>ModuleInfo</code> binary proto as defined
in
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/protobuf/stardoc_output.proto">stardoc_output.proto</a>
in the Bazel source tree.

<h4 id="starlark_doc_extract_implicit_outputs">Implicit output targets</h4>
        <ul>
          <li><code><var>name</var>.binaryproto</code> (the default output): A
            <code>ModuleInfo</code> binary proto.</li>
          <li><code><var>name</var>.textproto</code> (only built if explicitly requested): the text
            proto version of <code><var>name</var>.binaryproto</code>.</li>
        </ul>


Note: the exact output of this rule is not a stable public API. For example, the set of
natively-defined common rule attributes and their docstrings may change even with minor Bazel
releases. For this reason, documentation generated for user-defined rules is not stable across Bazel
releases, so we suggest taking care that any "golden tests" based on outputs of this rule are only
run with a single Bazel version.

(Ú

test_suite›

name(

aspect_hints

compatible_with

deprecation


features


licenses

package_metadata

restricted_to

tags

target_compatible_with


testonly

tests


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$implicit_tests"Å	
<p>
A <code>test_suite</code> defines a set of tests that are considered "useful" to humans. This
allows projects to define sets of tests, such as "tests you must run before checkin", "our
project's stress tests" or "all small tests." The <code>bazel test</code> command respects this sort
of organization: For an invocation like <code>bazel test //some/test:suite</code>, Bazel first
enumerates all test targets transitively included by the <code>//some/test:suite</code> target (we
call this "test_suite expansion"), then Bazel builds and tests those targets.
</p>

<h4 id="test_suite_examples">Examples</h4>

<p>A test suite to run all of the small tests in the current package.</p>
<pre class="code">
test_suite(
    name = "small_tests",
    tags = ["small"],
)
</pre>

<p>A test suite that runs a specified set of tests:</p>

<pre class="code">
test_suite(
    name = "smoke_tests",
    tests = [
        "system_unittest",
        "public_api_unittest",
    ],
)
</pre>

<p>A test suite to run all tests in the current package which are not flaky.</p>
<pre class="code">
test_suite(
    name = "non_flaky_test",
    tags = ["-flaky"],
)
</pre>

(Ø
constraint_setting˜

name(

aspect_hints

default_constraint_value

deprecation


features


licenses

tags


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs"ú
<p>This rule is used to introduce a new constraint type for which a platform may specify a value.
For instance, you might define a <code>constraint_setting</code> named "glibc_version" to represent
the capability for platforms to have different versions of the glibc library installed.

For more details, see the
<a href="https://bazel.build/docs/platforms">Platforms</a> page.

<p>Each <code>constraint_setting</code> has an extensible set of associated
<code>constraint_value</code>s. Usually these are defined in the same package, but sometimes a
different package will introduce new values for an existing setting. For instance, the predefined
setting <code>@platforms//cpu:cpu</code> can be extended with a custom value in order to
define a platform targeting an obscure cpu architecture.

(ﬁ
constraint_valueÛ

name(

aspect_hints

constraint_setting(

deprecation


features


licenses

tags


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs"—
This rule introduces a new value for a given constraint type.

For more details, see the
<a href="https://bazel.build/docs/platforms">Platforms</a> page.

<h4 id="constraint_value_examples">Example</h4>
<p>The following creates a new possible value for the predefined <code>constraint_value</code>
representing cpu architecture.
<pre class="code">
constraint_value(
    name = "mips",
    constraint_setting = "@platforms//cpu:cpu",
)
</pre>

Platforms can then declare that they have the <code>mips</code> architecture as an alternative to
<code>x86_64</code>, <code>arm</code>, and so on.

(∂3
platform˚

name(

aspect_hints

constraint_values

deprecation

exec_properties


features

flags


licenses

missing_toolchain_error
	
parents

required_settings

tags


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs

check_toolchain_types

allowed_toolchain_types"©0
<p>This rule defines a new platform -- a named collection of constraint choices
(such as cpu architecture or compiler version) describing an environment in
which part of the build may run.

For more details, see the <a href="/extending/platforms">Platforms</a> page.


<h4 id="platform_examples">Example</h4>
<p>
  This defines a platform that describes any environment running Linux on ARM.
</p>
<pre class="code">
platform(
    name = "linux_arm",
    constraint_values = [
        "@platforms//os:linux",
        "@platforms//cpu:arm",
    ],
)
</pre>

<h3 id="platform_flags">Platform Flags</h3>
<p>
  Platforms may use the <code>flags</code> attribute to specify a list of flags that will be added
  to the configuration whenever the platform is used as the target platform (i.e., as the value of
  the <code>--platforms</code> flag).
</p>

<p>
  Flags set from the platform effectively have the highest precedence and overwrite any previous
  value for that flag, from the command line, rc file, or transition.
</p>

<h4 id="platform_flags_examples">Example</h4>

<pre class="code">
platform(
    name = "foo",
    flags = [
        "--dynamic_mode=fully",
        "--//bool_flag",
        "--no//package:other_bool_flag",
    ],
)
</pre>

<p>
  This defines a platform named <code>foo</code>. When this is the target platform (either because
  the user specified <code>--platforms//:foo</code>, because a transition set the
  <code>//command_line_option:platforms</code> flag to <code>["//:foo"]</code>, or because
  <code>//:foo</code> was used as an execution platform), then the given flags will be set in the
  configuration.
</p>

<h4 id=platform_flags_repeated>Platforms and Repeatable Flags</h4>

<p>
  Some flags will accumulate values when they are repeated, such as <code>--features</code>,
  <code>--copt</code>, any Starlark flag created as <code>config.string(repeatable = True)</code>.
  These flags are not compatible with setting the flags from the platform: instead, all previous
  values will be removed and overwritten with the values from the platform.
</p>

<p>
  As an example, given the following platform, the invocation <code>build --platforms=//:repeat_demo
  --features feature_a --features feature_b</code> will end up with the value of the
  <code>--feature</code> flag being <code>["feature_c", "feature_d"]</code>, removing the features
  set on the command line.
</p>

<pre class="code">
platform(
    name = "repeat_demo",
    flags = [
        "--features=feature_c",
        "--features=feature_d",
    ],
)
</pre>

<p>
  For this reason, it is discouraged to use repeatable flags in the <code>flags</code> attribute.
</p>

<h3 id="platform_inheritance">Platform Inheritance</h3>
<p>
  Platforms may use the <code>parents</code> attribute to specify another platform that they will
  inherit constraint values from. Although the <code>parents</code> attribute takes a list, no
  more than a single value is currently supported, and specifying multiple parents is an error.
</p>

<p>
  When checking for the value of a constraint setting in a platform, first the values directly set
  (via the <code>constraint_values</code> attribute) are checked, and then the constraint values on
  the parent. This continues recursively up the chain of parent platforms. In this manner, any
  values set directly on a platform will override the values set on the parent.
</p>

<p>
  Platforms inherit the <code>exec_properties</code> attribute from the parent platform.
  The dictionary entries in <code>exec_properties</code> of the parent and child platforms
  will be combined.
  If the same key appears in both the parent's and the child's <code>exec_properties</code>,
  the child's value will be used. If the child platform specifies an empty string as a value, the
  corresponding property will be unset.
</p>

<h4 id="platform_inheritance_examples">Example: Constraint Values</h4>
<pre class="code">
platform(
    name = "parent",
    constraint_values = [
        "@platforms//os:linux",
        "@platforms//cpu:arm",
    ],
)
platform(
    name = "child_a",
    parents = [":parent"],
    constraint_values = [
        "@platforms//cpu:x86_64",
    ],
)
platform(
    name = "child_b",
    parents = [":parent"],
)
</pre>

<p>
  In this example, the child platforms have the following properties:

  <ul>
    <li>
      <code>child_a</code> has the constraint values <code>@platforms//os:linux</code> (inherited
      from the parent) and <code>@platforms//cpu:x86_64</code> (set directly on the platform).
    </li>
    <li>
      <code>child_b</code> inherits all constraint values from the parent, and doesn't set any of
      its own.
    </li>
  </ul>
</p>

<h4 id="platform_inheritance_exec_examples">Example: Execution properties</h4>
<pre class="code">
platform(
    name = "parent",
    exec_properties = {
      "k1": "v1",
      "k2": "v2",
    },
)
platform(
    name = "child_a",
    parents = [":parent"],
)
platform(
    name = "child_b",
    parents = [":parent"],
    exec_properties = {
      "k1": "child"
    }
)
platform(
    name = "child_c",
    parents = [":parent"],
    exec_properties = {
      "k1": ""
    }
)
platform(
    name = "child_d",
    parents = [":parent"],
    exec_properties = {
      "k3": "v3"
    }
)
</pre>

<p>
  In this example, the child platforms have the following properties:

  <ul>
    <li>
      <code>child_a</code> inherits the "exec_properties" of the parent and does not set its own.
    </li>
    <li>
      <code>child_b</code> inherits the parent's <code>exec_properties</code> and overrides the
      value of <code>k1</code>. Its <code>exec_properties</code> will be:
      <code>{ "k1": "child", "k2": "v2" }</code>.
    </li>
    <li>
      <code>child_c</code> inherits the parent's <code>exec_properties</code> and unsets
      <code>k1</code>. Its <code>exec_properties</code> will be:
      <code>{ "k2": "v2" }</code>.
    </li>
    <li>
      <code>child_d</code> inherits the parent's <code>exec_properties</code> and adds a new
      property. Its <code>exec_properties</code> will be:
      <code>{ "k1": "v1",  "k2": "v2", "k3": "v3" }</code>.
    </li>
  </ul>
</p>

(Â
	toolchain˙

name(

aspect_hints

deprecation

exec_compatible_with


features


licenses

package_metadata

tags

target_compatible_with

target_settings


testonly

	toolchain(

toolchain_type(
!
use_target_platform_constraints


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs"ÿ
<p>This rule declares a specific toolchain's type and constraints so that it can be selected
during toolchain resolution. See the
<a href="https://bazel.build/docs/toolchains">Toolchains</a> page for more
details.

(û
toolchain_typeß

name(

aspect_hints

compatible_with

deprecation


features

no_match_error

package_metadata

restricted_to

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies"ﬂ
<p>
  This rule defines a new type of toolchain -- a simple target that represents a class of tools that
  serve the same role for different platforms.
</p>

<p>
  See the <a href="/docs/toolchains">Toolchains</a> page for more details.
</p>

<h4 id="toolchain_type_examples">Example</h4>
<p>
  This defines a toolchain type for a custom rule.
</p>
<pre class="code">
toolchain_type(
    name = "bar_toolchain_type",
)
</pre>

<p>
  This can be used in a bzl file.
</p>
<pre class="code">
bar_binary = rule(
    implementation = _bar_binary_impl,
    attrs = {
        "srcs": attr.label_list(allow_files = True),
        ...
        # No `_compiler` attribute anymore.
    },
    toolchains = ["//bar_tools:toolchain_type"]
)
</pre>
(©A built-in module to support native rules and other package helper functions. All native rules appear as functions in this module, e.g. <code>native.cc_library</code>. Note that the native module is only available in the loading phase (i.e. for macros, not for rule implementations). Attributes will ignore <code>None</code> values, and treat them as if the attribute was unset.<br>The following functions are also available:
Ô	
platform_common∏
ConstraintSettingInfoProvider"îThe constructor/key for the <a href='../providers/ConstraintSettingInfo.html'>ConstraintSettingInfo</a> provider.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>≤
ConstraintValueInfoProvider"êThe constructor/key for the <a href='../providers/ConstraintValueInfo.html'>ConstraintValueInfo</a> provider.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>ù
PlatformInfoProvider"ÇThe constructor/key for the <a href='../providers/PlatformInfo.html'>PlatformInfo</a> provider.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>ë
TemplateVariableInfoProvider"oThe constructor/key for the <a href='../providers/TemplateVariableInfo.html'>TemplateVariableInfo</a> provider.|
ToolchainInfoProvider"aThe constructor/key for the <a href='../providers/ToolchainInfo.html'>ToolchainInfo</a> provider.:Functions for Starlark to interact with the platform APIs.
Á
proto≤
encode_text)

xstructure; or NativeInfo(string"˜Returns the struct argument's encoding as a text-format protocol message.
The data structure must be recursively composed of strings, ints, floats, or bools, or structs, sequences, and dicts of these types.
<p>A struct is converted to a message. Fields are emitted in name order.
Each struct field whose value is None is ignored.
<p>A sequence (such as a list or tuple) is converted to a repeated field.
Its elements must not be sequences or dicts.
<p>A dict is converted to a repeated field of messages with fields named 'key' and 'value'.
Entries are emitted in iteration (insertion) order.
The dict's keys must be strings or ints, and its values must not be sequences or dicts.
Examples:<br><pre class=language-python>proto.encode_text(struct(field=123))
# field: 123

proto.encode_text(struct(field=True))
# field: true

proto.encode_text(struct(field=[1, 2, 3]))
# field: 1
# field: 2
# field: 3

proto.encode_text(struct(field='text', ignored_field=None))
# field: "text"

proto.encode_text(struct(field=struct(inner_field='text', ignored_field=None)))
# field {
#   inner_field: "text"
# }

proto.encode_text(struct(field=[struct(inner_field=1), struct(inner_field=2)]))
# field {
#   inner_field: 1
# }
# field {
#   inner_field: 2
# }

proto.encode_text(struct(field=struct(inner_field=struct(inner_inner_field='text'))))
# field {
#    inner_field {
#     inner_inner_field: "text"
#   }
# }

proto.encode_text(struct(foo={4: 3, 2: 1}))
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre>)A module for protocol message processing.
æ
testing|
ExecutionInfoExecutionInfo"\<a href='../providers/ExecutionInfo.html'>testing.ExecutionInfo</a> provider key/constructorÆ
TestEnvironment◊
’
environment3<a class="anchor" href="../core/dict.html">dict</a>éA map of string keys and values that represent environment variables and their values. These will be made available during the test execution.(
Ë
inherited_environments<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s’A sequence of names of environment variables. These variables are made available during the test execution with their current value taken from the shell environment. If a variable is contained in both <code>environment</code> and <code>inherited_environment</code>, the value inherited from the shell environment will take precedence if set."[]RunEnvironmentInfo"¿<b>Deprecated: Use RunEnvironmentInfo instead.</b> Creates a new test environment provider. Use this provider to specify extra environment variables to be made available during test execution.ƒ
analysis_test˛
•
name7<a class="anchor" href="../core/string.html">string</a>bName of the target. It should be a Starlark identifier, matching pattern '[A-Za-z_][A-Za-z0-9_]*'.(
Æ
implementation;<a class="anchor" href="../core/function.html">function</a>‹The Starlark function implementing this analysis test. It must have exactly one parameter: <a href="../builtins/ctx.html">ctx</a>. The function is called during the analysis phase. It can access the attributes declared by <code>attrs</code> and populated via <code>attr_values</code>. The implementation function may not register actions. Instead, it must register a pass/fail result via providing <a href='../providers/AnalysisTestResultInfo.html'>AnalysisTestResultInfo</a>.(
Ω
attrs3<a class="anchor" href="../core/dict.html">dict</a>˙Dictionary declaring the attributes. See the <a href="../globals/bzl.html#rule">rule</a> call. Attributes are allowed to use configuration transitions defined using <a  href="../globals/bzl.html#analysis_test_transition">analysis_test_transition</a>."{}
‰
	fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s^List of configuration fragments that are available to the implementation of the analysis test."[]
≠

toolchains7<a class="anchor" href="../core/list.html">sequence</a>bThe set of toolchains the test requires. See the <a href="../globals/bzl.html#rule">rule</a> call."[]
¡
attr_valueso<a class="anchor" href="../core/dict.html">dict</a> of <a class="anchor" href="../core/string.html">string</a>s=Dictionary of attribute values to pass to the implementation."{}NoneType"±Creates a new analysis test target. <p>The number of transitive dependencies of the test are limited. The limit is controlled by <code>--analysis_testing_deps_limit</code> flag.=Helper methods for Starlark to access testing infrastructure.
¬
boolπA type to represent booleans. There are only two possible values: True and False. Any value can be converted to a boolean using the <a href="../globals/all.html#bool">bool</a> function.
T
builtin_function_or_method6The type of a built-in function, defined by Java code.
˝%
dict:
clear
NoneType"%Remove all items from the dictionary.Ê
get~

keyThe key to look for.(
T
defaultCThe default value to use (instead of None) if the key is not found."Noneunknown"ﬁReturns the value for <code>key</code> if <code>key</code> is in the dictionary, else <code>default</code>. If <code>default</code> is not given, it defaults to <code>None</code>, so that this method never throws an error.ù
itemslist"ãReturns the list of key-value tuples:<pre class="language-python">{2: "a", 4: "b", 1: "c"}.items() == [(2, "a"), (4, "b"), (1, "c")]</pre>
y
keyslist"iReturns the list of keys:<pre class="language-python">{2: "a", 4: "b", 1: "c"}.keys() == [2, 4, 1]</pre>
∆
popW

keyThe key.(
9
default%a default value if the key is absent."unboundunknown"ÂRemoves a <code>key</code> from the dict, and returns the associated value. If no entry with that key was found, remove nothing and return the specified <code>default</code> value; if no default value was specified, fail instead.ë
popitemtuple"¸Remove and return the first <code>(key, value)</code> pair from the dictionary. <code>popitem</code> is useful to destructively iterate over a dictionary, as often used in set algorithms. If the dictionary is empty, the <code>popitem</code> call fails.≠

setdefaultT

keyThe key.(
6
default%a default value if the key is absent."Noneunknown"»If <code>key</code> is in the dictionary, return its value. If not, insert key with a value of <code>default</code> and return <code>default</code>. <code>default</code> defaults to <code>None</code>.‡
updateº
}
pairspEither a dictionary or a list of entries. Entries must be tuples or lists with exactly two elements: key, value."[]
1
**kwargs!Dictionary of additional entries.(8NoneType"ñUpdates the dictionary first with the optional positional argument, <code>pairs</code>,  then with the optional keyword arguments
If the positional argument is present, it must be a dict, iterable, or None.
If it is a dict, then its key/value pairs are inserted into this dict. If it is an iterable, it must provide a sequence of pairs (or other iterables of length 2), each of which is treated as a key/value pair to be inserted.
Each keyword argument <code>name=value</code> causes the name/value pair to be inserted into this dict.Ö
valueslist"sReturns the list of values:<pre class="language-python">{2: "a", 4: "b", 1: "c"}.values() == ["a", "b", "c"]</pre>
ºdict is a built-in type representing an associative mapping or <i>dictionary</i>. A dictionary supports indexing using <code>d[k]</code> and key membership testing using <code>k in d</code>; both operations take constant time. Unfrozen dictionaries are mutable, and may be updated by assigning to <code>d[k]</code> or by calling certain methods. Dictionaries are iterable; iteration yields the sequence of keys in insertion order. Iteration order is unaffected by updating the value associated with an existing key, but is affected by removing then reinserting a key.
<pre>d = {0: "x", 2: "z", 1: "y"}
[k for k in d]  # [0, 2, 1]
d.pop(2)
d[0], d[2] = "a", "b"
0 in d, "a" in d  # (True, False)
[(k, v) for k, v in d.items()]  # [(0, "a"), (1, "y"), (2, "b")]
</pre>
<p>There are four ways to construct a dictionary:
<ol>
<li>A dictionary expression <code>{k: v, ...}</code> yields a new dictionary with the specified key/value entries, inserted in the order they appear in the expression. Evaluation fails if any two key expressions yield the same value.
<li>A dictionary comprehension <code>{k: v for vars in seq}</code> yields a new dictionary into which each key/value pair is inserted in loop iteration order. Duplicates are permitted: the first insertion of a given key determines its position in the sequence, and the last determines its associated value.
<pre class="language-python">
{k: v for k, v in (("a", 0), ("b", 1), ("a", 2))}  # {"a": 2, "b": 1}
{i: 2*i for i in range(3)}  # {0: 0, 1: 2, 2: 4}
</pre>
<li>A call to the built-in <a href="../globals/all.html#dict">dict</a> function returns a dictionary containing the specified entries, which are inserted in argument order, positional arguments before named. As with comprehensions, duplicate keys are permitted.
<li>The union expression <code>x | y</code> yields a new dictionary by combining two existing dictionaries. If the two dictionaries have a key <code>k</code> in common, the right hand side dictionary's value of the key (in other words, <code>y[k]</code>) wins. The <code>|=</code> variant of the union operator modifies a dictionary in-place. Example:<br><pre class=language-python>d = {"foo": "FOO", "bar": "BAR"} | {"foo": "FOO2", "baz": "BAZ"}
# d == {"foo": "FOO2", "bar": "BAR", "baz": "BAZ"}
d = {"a": 1, "b": 2}
d |= {"b": 3, "c": 4}
# d == {"a": 1, "b": 3, "c": 4}</pre></ol>
8
float/The type of floating-point numbers in Starlark.
7
function+The type of functions declared in Starlark.
µ
int≠The type of integers in Starlark. Starlark integers may be of any magnitude; arithmetic is exact. Examples of integer expressions:<br><pre class="language-python">153
0x2A  # hexadecimal literal
0o54  # octal literal
23 * 2 + 5
100 / -7
100 % -7  # -5 (unlike in some other languages)
int("18")
</pre>
€
json‹	
decodeø
V
x7<a class="anchor" href="../core/string.html">string</a>JSON string to decode.(
\
defaultHIf specified, the value to return when <code>x</code> cannot be decoded."unboundunknown"èThe decode function has one required positional parameter: a JSON string.
It returns the Starlark value that the string denotes.
<ul><li><code>"null"</code>, <code>"true"</code> and <code>"false"</code> are parsed as <code>None</code>, <code>True</code>, and <code>False</code>.
<li>Numbers are parsed as int, or as a float if they contain a decimal point or an exponent. Although JSON has no syntax  for non-finite values, very large values may be decoded as infinity.
<li>a JSON object is parsed as a new unfrozen Starlark dict. If the same key string occurs more than once in the object, the last value for the key is kept.
<li>a JSON array is parsed as new unfrozen Starlark list.
</ul>
If <code>x</code> is not a valid JSON encoding and the optional <code>default</code> parameter is specified (including specified as <code>None</code>), this function returns the <code>default</code> value.
If <code>x</code> is not a valid JSON encoding and the optional <code>default</code> parameter is <em>not</em> specified, this function fails.ﬂ
encode

x(string"√<p>The encode function accepts one required positional argument, which it converts to JSON by cases:
<ul>
<li>None, True, and False are converted to 'null', 'true', and 'false', respectively.
<li>An int, no matter how large, is encoded as a decimal integer. Some decoders may not be able to decode very large integers.
<li>A float is encoded using a decimal point or an exponent or both, even if its numeric value is an integer. It is an error to encode a non-finite  floating-point value.
<li>A string value is encoded as a JSON string literal that denotes the value.  Each unpaired surrogate is replaced by U+FFFD.
<li>A dict is encoded as a JSON object, in lexicographical key order.  It is an error if any key is not a string.
<li>A list or tuple is encoded as a JSON array.
<li>A struct-like value is encoded as a JSON object, in field name order.
</ul>
An application-defined type may define its own JSON encoding.
Encoding any other value yields an error.
–
encode_indentü

x(
E
prefix7<a class="anchor" href="../core/string.html">string</a>"''
G
indent7<a class="anchor" href="../core/string.html">string</a>"'\t'string"úThe encode_indent function is equivalent to <code>json.indent(json.encode(x), ...)</code>. See <code>indent</code> for description of formatting parameters.Å
indentÿ
>
s7<a class="anchor" href="../core/string.html">string</a>(
E
prefix7<a class="anchor" href="../core/string.html">string</a>"''
G
indent7<a class="anchor" href="../core/string.html">string</a>"'\t'string"õThe indent function returns the indented form of a valid JSON-encoded string.
Each array element or object field appears on a new line, beginning with the prefix string followed by one or more copies of the indent string, according to its nesting depth.
The function accepts one required positional parameter, the JSON string,
and two optional keyword-only string parameters, prefix and indent,
that specify a prefix of each new line, and the unit of indentation.
If the input is not valid, the function may fail or return invalid output.
;Module json is a Starlark module of JSON-related functions.
ì
list]
append-
!
itemItem to add at the end.(NoneType"$Adds an item to the end of the list.:
clear
NoneType"%Removes all the elements of the list.k
extend9
-
itemsiterableItems to add at the end.(NoneType"&Adds all items to the end of the list.Å
indexã

xThe object to search.(
t
start1<a class="anchor" href="../core/int.html">int</a>/The start index of the list portion to inspect."unbound
p
end1<a class="anchor" href="../core/int.html">int</a>-The end index of the list portion to inspect."unboundint"jReturns the index in the list of the first item whose value is x. It is an error if there is no such item.Ø
insert
^
index1<a class="anchor" href="../core/int.html">int</a> The index of the given position.(

item	The item.(NoneType"$Inserts an item at a given position.É
pop]
R
i1<a class="anchor" href="../core/int.html">int</a>The index of the item."-1unknown"úRemoves the item at the given position in the list, and returns it. If no <code>index</code> is specified, it removes and returns the last item in the list.ì
remove(

xThe object to remove.(NoneType"_Removes the first item from the list whose value is x. It is an error if there is no such item.∞The built-in list type. Example list expressions:<br><pre class=language-python>x = [1, 2, 3]</pre>Accessing elements is possible using indexing (starts from <code>0</code>):<br><pre class=language-python>e = x[1]   # e == 2</pre>Lists support the <code>+</code> operator to concatenate two lists. Example:<br><pre class=language-python>x = [1, 2] + [3, 4]   # x == [1, 2, 3, 4]
x = ["a", "b"]
x += ["c"]            # x == ["a", "b", "c"]</pre>Similar to strings, lists support slice operations:<pre class=language-python>['a', 'b', 'c', 'd'][1:3]   # ['b', 'c']
['a', 'b', 'c', 'd'][::2]  # ['a', 'c']
['a', 'b', 'c', 'd'][3:0:-1]  # ['d', 'c', 'b']</pre>Lists are mutable, as in Python.
≠
range£A language built-in type to support ranges. Example of range literal:<br><pre class=language-python>x = range(1, 10, 3)</pre>Accessing elements is possible using indexing (starts from <code>0</code>):<br><pre class=language-python>e = x[1]   # e == 2</pre>Ranges do not support the <code>+</code> operator for concatenation.Similar to strings, ranges support slice operations:<pre class=language-python>range(10)[1:3]   # range(1, 3)
range(10)[::2]  # range(0, 10, 2)
range(10)[3:0:-1]  # range(3, 0, -1)</pre>Ranges are immutable, as in Python 3.
ßf
set”
add(

elementElement to add.(NoneType"°Adds an element to the set.

<p>It is permissible to <code>add</code> a value already present in the set; this leaves the set
unchanged.

<p>If you need to add multiple elements to a set, see <a href="#update"><code>update</code></a> or
the <code>|=</code> augmented assignment operation.
9
clear
NoneType"$Removes all the elements of the set.µ

difference7
0
*others!Collections of hashable elements.(0set"ÌReturns a new mutable set containing the difference of this set with others.

<p>If <code>s</code> and <code>t</code> are sets, <code>s.difference(t)</code> is equivalent to
<code>s - t</code>; however, note that the <code>-</code> operation requires both sides to be sets,
while the <code>difference</code> method also accepts sequences and dicts.

<p>It is permissible to call <code>difference</code> without any arguments; this returns a copy of
the set.

<p>For example,
<pre class=language-python>
set([1, 2, 3]).difference([2])             # set([1, 3])
set([1, 2, 3]).difference([0, 1], [3, 4])  # set([2])
</pre>

difference_update<
0
*others!Collections of hashable elements.(0NoneType"úRemoves any elements found in any others from this set.

<p>If <code>s</code> and <code>t</code> are sets, <code>s.difference_update(t)</code> is equivalent
to <code>s -= t</code>; however, note that the <code>-=</code> augmented assignment requires both
sides to be sets, while the <code>difference_update</code> method also accepts sequences and dicts.

<p>It is permissible to call <code>difference_update</code> without any arguments; this leaves the
set unchanged.

<p>For example,
<pre class=language-python>
s = set([1, 2, 3, 4])
s.difference_update([2])             # None; s is set([1, 3, 4])
s.difference_update([0, 1], [4, 5])  # None; s is set([3])
</pre>
¨
discard>
2
element%Element to discard. Must be hashable.(NoneType"‡Removes an element from the set if it is present.

<p>It is permissible to <code>discard</code> a value not present in the set; this leaves the set
unchanged. If you want to fail on an attempt to remove a non-present element, use
<a href="#remove"><code>remove</code></a> instead. If you need to remove multiple elements from a
set, see <a href="#difference_update"><code>difference_update</code></a> or the <code>-=</code>
augmented assignment operation.

<p>For example,
<pre class=language-python>
s = set(["x", "y"])
s.discard("y")  # None; s == set(["x"])
s.discard("y")  # None; s == set(["x"])
</pre>
»
intersection7
0
*others!Collections of hashable elements.(0set"˛Returns a new mutable set containing the intersection of this set with others.

<p>If <code>s</code> and <code>t</code> are sets, <code>s.intersection(t)</code> is equivalent to
<code>s &amp; t</code>; however, note that the <code>&amp;</code> operation requires both sides to
be sets, while the <code>intersection</code> method also accepts sequences and dicts.

<p>It is permissible to call <code>intersection</code> without any arguments; this returns a copy of
the set.

<p>For example,
<pre class=language-python>
set([1, 2]).intersection([2, 3])             # set([2])
set([1, 2, 3]).intersection([0, 1], [1, 2])  # set([1])
</pre>
Ö
intersection_update<
0
*others!Collections of hashable elements.(0NoneType"ØRemoves any elements not found in all others from this set.

<p>If <code>s</code> and <code>t</code> are sets, <code>s.intersection_update(t)</code> is
equivalent to <code>s &amp;= t</code>; however, note that the <code>&amp;=</code> augmented
assignment requires both sides to be sets, while the <code>intersection_update</code> method also
accepts sequences and dicts.

<p>It is permissible to call <code>intersection_update</code> without any arguments; this leaves the
set unchanged.

<p>For example,
<pre class=language-python>
s = set([1, 2, 3, 4])
s.intersection_update([0, 1, 2])       # None; s is set([1, 2])
s.intersection_update([0, 1], [1, 2])  # None; s is set([1])
</pre>
±

isdisjoint5
-
other"A collection of hashable elements.(bool"ÎReturns true if this set has no elements in common with another.

<p>For example,
<pre class=language-python>
set([1, 2]).isdisjoint([3, 4])  # True
set().isdisjoint(set())         # True
set([1, 2]).isdisjoint([2, 3])  # False
</pre>
Â
issubset5
-
other"A collection of hashable elements.(bool"°Returns true of this set is a subset of another.

<p>Note that a set is always considered to be a subset of itself.

<p>For example,
<pre class=language-python>
set([1, 2]).issubset([1, 2, 3])  # True
set([1, 2]).issubset([1, 2])     # True
set([1, 2]).issubset([2, 3])     # False
</pre>
˙

issuperset5
-
other"A collection of hashable elements.(bool"¥Returns true of this set is a superset of another.

<p>Note that a set is always considered to be a superset of itself.

<p>For example,
<pre class=language-python>
set([1, 2, 3]).issuperset([1, 2])     # True
set([1, 2, 3]).issuperset([1, 2, 3])  # True
set([1, 2, 3]).issuperset([2, 3, 4])  # False
</pre>
Ú
pop	unknown"ﬂRemoves and returns the first element of the set (in iteration order, which is the order in which
elements were first added to the set).

<p>Fails if the set is empty.

<p>For example,
<pre class=language-python>
s = set([3, 1, 2])
s.pop()  # 3; s == set([1, 2])
s.pop()  # 1; s == set([2])
s.pop()  # 2; s == set()
s.pop()  # error: empty set
</pre>
§
removeY
M
element@Element to remove. Must be an element of the set (and hashable).(NoneType"æRemoves an element, which must be present in the set, from the set.

<p><code>remove</code> fails if the element was not present in the set. If you don't want to fail on
an attempt to remove a non-present element, use <a href="#discard"><code>discard</code></a> instead.
If you need to remove multiple elements from a set, see
<a href="#difference_update"><code>difference_update</code></a> or the <code>-=</code> augmented
assignment operation.
ÿ
symmetric_difference4
-
other"A collection of hashable elements.(set"âReturns a new mutable set containing the symmetric difference of this set with another collection of
hashable elements.

<p>If <code>s</code> and <code>t</code> are sets, <code>s.symmetric_difference(t)</code> is
equivalent to <code>s ^ t</code>; however, note that the <code>^</code> operation requires both
sides to be sets, while the <code>symmetric_difference</code> method also accepts a sequence or a
dict.

<p>For example,
<pre class=language-python>
set([1, 2]).symmetric_difference([2, 3])  # set([1, 3])
</pre>
å
symmetric_difference_update9
-
other"A collection of hashable elements.(NoneType"±Returns a new mutable set containing the symmetric difference of this set with another collection of
hashable elements.

<p>If <code>s</code> and <code>t</code> are sets, <code>s.symmetric_difference_update(t)</code> is
equivalent to `s ^= t<code>; however, note that the </code>^=` augmented assignment requires both
sides to be sets, while the <code>symmetric_difference_update</code> method also accepts a sequence
or a dict.

<p>For example,
<pre class=language-python>
s = set([1, 2])
s.symmetric_difference_update([2, 3])  # None; s == set([1, 3])
</pre>
¨
union7
0
*others!Collections of hashable elements.(0set"ÈReturns a new mutable set containing the union of this set with others.

<p>If <code>s</code> and <code>t</code> are sets, <code>s.union(t)</code> is equivalent to
<code>s | t</code>; however, note that the <code>|</code> operation requires both sides to be sets,
while the <code>union</code> method also accepts sequences and dicts.

<p>It is permissible to call <code>union</code> without any arguments; this returns a copy of the
set.

<p>For example,
<pre class=language-python>
set([1, 2]).union([2, 3])                    # set([1, 2, 3])
set([1, 2]).union([2, 3], {3: "a", 4: "b"})  # set([1, 2, 3, 4])
</pre>
ü
update<
0
*others!Collections of hashable elements.(0NoneType"÷Adds the elements found in others to this set.

<p>For example,
<pre class=language-python>
s = set()
s.update([1, 2])          # None; s is set([1, 2])
s.update([2, 3], [3, 4])  # None; s is set([1, 2, 3, 4])
</pre>

<p>If <code>s</code> and <code>t</code> are sets, <code>s.update(t)</code> is equivalent to
<code>s |= t</code>; however, note that the <code>|=</code> augmented assignment requires both sides
to be sets, while the <code>update</code> method also accepts sequences and dicts.

<p>It is permissible to call <code>update</code> without any arguments; this leaves the set
unchanged.
Ò"The built-in set type. A set is a mutable collection of unique values &ndash; the set's
<em>elements</em>. The <a href="../globals/all#type">type name</a> of a set is <code>"set"</code>.

<p>Sets provide constant-time operations to insert, remove, or check for the presence of a value.
Sets are implemented using a hash table, and therefore, just like keys of a
<a href="../dict">dictionary</a>, elements of a set must be hashable. A value may be used as an
element of a set if and only if it may be used as a key of a dictionary.

<p>Sets may be constructed using the <a href="../globals/all#set"><code>set()</code></a> built-in
function, which returns a new set containing the unique elements of its optional argument, which
must be an iterable. Calling <code>set()</code> without an argument constructs an empty set. Sets
have no literal syntax.

<p>The <code>in</code> and <code>not in</code> operations check whether a value is (or is not) in a
set:

<pre class=language-python>
s = set(["a", "b", "c"])
"a" in s  # True
"z" in s  # False
</pre>

<p>A set is iterable, and thus may be used as the operand of a <code>for</code> loop, a list
comprehension, and the various built-in functions that operate on iterables. Its length can be
retrieved using the <a href="../globals/all#len"><code>len()</code></a> built-in function, and the
order of iteration is the order in which elements were first added to the set:

<pre class=language-python>
s = set(["z", "y", "z", "y"])
len(s)       # prints 2
s.add("x")
len(s)       # prints 3
for e in s:
    print e  # prints "z", "y", "x"
</pre>

<p>A set used in Boolean context is true if and only if it is non-empty.

<pre class=language-python>
s = set()
"non-empty" if s else "empty"  # "empty"
t = set(["x", "y"])
"non-empty" if t else "empty"  # "non-empty"
</pre>

<p>Sets may be compared for equality or inequality using <code>==</code> and <code>!=</code>. A set
<code>s</code> is equal to <code>t</code> if and only if <code>t</code> is a set containing the same
elements; iteration order is not significant. In particular, a set is <em>not</em> equal to the list
of its elements. Sets are not ordered with respect to other sets, and an attempt to compare two sets
using <code>&lt;</code>, <code>&lt;=</code>, <code>&gt;</code>, <code>&gt;=</code>, or to sort a
sequence of sets, will fail.

<pre class=language-python>
set() == set()              # True
set() != []                 # True
set([1, 2]) == set([2, 1])  # True
set([1, 2]) != [1, 2]       # True
</pre>

<p>The <code>|</code> operation on two sets returns the union of the two sets: a set containing the
elements found in either one or both of the original sets.

<pre class=language-python>
set([1, 2]) | set([3, 2])  # set([1, 2, 3])
</pre>

<p>The <code>&amp;</code> operation on two sets returns the intersection of the two sets: a set
containing only the elements found in both of the original sets.

<pre class=language-python>
set([1, 2]) &amp; set([2, 3])  # set([2])
set([1, 2]) &amp; set([3, 4])  # set()
</pre>

<p>The <code>-</code> operation on two sets returns the difference of the two sets: a set containing
the elements found in the left-hand side set but not the right-hand side set.

<pre class=language-python>
set([1, 2]) - set([2, 3])  # set([1])
set([1, 2]) - set([3, 4])  # set([1, 2])
</pre>

<p>The <code>^</code> operation on two sets returns the symmetric difference of the two sets: a set
containing the elements found in exactly one of the two original sets, but not in both.

<pre class=language-python>
set([1, 2]) ^ set([2, 3])  # set([1, 3])
set([1, 2]) ^ set([3, 4])  # set([1, 2, 3, 4])
</pre>

<p>In each of the above operations, the elements of the resulting set retain their order from the
two operand sets, with all elements that were drawn from the left-hand side ordered before any
element that was only present in the right-hand side.

<p>The corresponding augmented assignments, <code>|=</code>, <code>&amp;=</code>, <code>-=</code>,
and <code>^=</code>, modify the left-hand set in place.

<pre class=language-python>
s = set([1, 2])
s |= set([2, 3, 4])     # s now equals set([1, 2, 3, 4])
s &amp;= set([0, 1, 2, 3])  # s now equals set([1, 2, 3])
s -= set([0, 1])        # s now equals set([2, 3])
s ^= set([3, 4])        # s now equals set([2, 4])
</pre>

<p>Like all mutable values in Starlark, a set can be frozen, and once frozen, all subsequent
operations that attempt to update it will fail.

ﬁa
string∞

capitalizestring"óReturns a copy of the string with its first character (if any) capitalized and the rest lowercased. This method does not support non-ascii characters. —
countÎ
Y
sub7<a class="anchor" href="../core/string.html">string</a>The substring to count.(
{
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>&Restrict to search from this position."0
ã
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>5optional position before which to restrict to search."Noneint"ŸReturns the number of (non-overlapping) occurrences of substring <code>sub</code> in string, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive.◊
elems
sequence"¡Returns an iterable value containing successive 1-element substrings of the string. Equivalent to <code>[s[i] for i in range(len(s))]</code>, except that the returned value might not be a list. 
endswithˆ

sub≠<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../core/tuple.html">tuple</a> of <a class="anchor" href="../core/string.html">string</a>s7The suffix (or tuple of alternative suffixes) to match.(
u
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code> Test beginning at this position."0
É
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>-optional position at which to stop comparing."Nonebool"ƒReturns True if the string ends with <code>sub</code>, otherwise False, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive. 
findÍ
X
sub7<a class="anchor" href="../core/string.html">string</a>The substring to find.(
{
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>&Restrict to search from this position."0
ã
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>5optional position before which to restrict to search."Noneint"‘Returns the first index where <code>sub</code> is found, or -1 if no such index exists, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive.◊
formatW
!
*argsList of arguments."()0
*
**kwargsDictionary of arguments."{}8string"ÛPerform string interpolation. Format strings contain replacement fields surrounded by curly braces <code>&#123;&#125;</code>. Anything that is not contained in braces is considered literal text, which is copied unchanged to the output.If you need to include a brace character in the literal text, it can be escaped by doubling: <code>&#123;&#123;</code> and <code>&#125;&#125;</code>A replacement field can be either a name, a number, or empty. Values are converted to strings using the <a href="../globals/all.html#str">str</a> function.<pre class="language-python"># Access in order:
"&#123;&#125; < &#123;&#125;".format(4, 5) == "4 < 5"
# Access by position:
"{1}, {0}".format(2, 1) == "1, 2"
# Access by name:
"x{key}x".format(key = 2) == "x2x"</pre>
◊
indexÍ
X
sub7<a class="anchor" href="../core/string.html">string</a>The substring to find.(
{
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>&Restrict to search from this position."0
ã
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>5optional position before which to restrict to search."Noneint"‡Returns the first index where <code>sub</code> is found, or raises an error if no such  index exists, optionally restricting to <code>[start:end]</code><code>start</code> being inclusive and <code>end</code> being exclusive.É
isalnumbool"pReturns True if all characters in the string are alphanumeric ([a-zA-Z0-9]) and there is at least one character.~
isalphabool"kReturns True if all characters in the string are alphabetic ([a-zA-Z]) and there is at least one character.w
isdigitbool"dReturns True if all characters in the string are digits ([0-9]) and there is at least one character.x
islowerbool"eReturns True if all cased characters in the string are lowercase and there is at least one character.|
isspacebool"iReturns True if all characters are white space characters and the string contains at least one character.ì
istitlebool"ˇReturns True if the string is in title case and it contains at least one character. This means that every uppercase character must follow an uncased one (e.g. whitespace) and every lowercase character must follow a cased one (e.g. uppercase or lowercase).x
isupperbool"eReturns True if all cased characters in the string are uppercase and there is at least one character.æ
joinr
h
elementsDiterable of <a class="anchor" href="../core/string.html">string</a>sThe objects to join.(string"¡Returns a string in which the string elements of the argument have been joined by this string as a separator. Example:<br><pre class="language-python">"|".join(["a", "b", "c"]) == "a|b|c"</pre>A
lowerstring".Returns the lower case version of this string.®
lstripù
í
charsM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>4The characters to remove, or all whitespace if None."Nonestring"˝Returns a copy of the string where leading characters that appear in <code>chars</code> are removed. Note that <code>chars</code> is not a prefix: all combinations of its value are removed:<pre class="language-python">"abcba".lstrip("ba") == "cba"</pre>˛
	partitionb
Y
sep7<a class="anchor" href="../core/string.html">string</a>The string to split on.(tuple"åSplits the input string at the first occurrence of the separator <code>sep</code> and returns the resulting partition as a three-element tuple of the form (before, separator, after). If the input string does not contain the separator, partition returns (self, '', '').¸
removeprefixo
e
prefix7<a class="anchor" href="../core/string.html">string</a> The prefix to remove if present.(string"{If the string starts with <code>prefix</code>, returns a new string with the prefix removed. Otherwise, returns the string.˙
removesuffixo
e
suffix7<a class="anchor" href="../core/string.html">string</a> The suffix to remove if present.(string"yIf the string ends with <code>suffix</code>, returns a new string with the suffix removed. Otherwise, returns the string.≤
replaceÁ
\
old7<a class="anchor" href="../core/string.html">string</a>The string to be replaced.(
]
new7<a class="anchor" href="../core/string.html">string</a>The string to replace with.(
ü
count1<a class="anchor" href="../core/int.html">int</a>_The maximum number of replacements. If omitted, or if the value is negative, there is no limit."-1string"ºReturns a copy of the string in which the occurrences of <code>old</code> have been replaced with <code>new</code>, optionally restricting the number of replacements to <code>count</code>. 
rfindÍ
X
sub7<a class="anchor" href="../core/string.html">string</a>The substring to find.(
{
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>&Restrict to search from this position."0
ã
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>5optional position before which to restrict to search."Noneint"”Returns the last index where <code>sub</code> is found, or -1 if no such index exists, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive.ÿ
rindexÍ
X
sub7<a class="anchor" href="../core/string.html">string</a>The substring to find.(
{
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>&Restrict to search from this position."0
ã
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>5optional position before which to restrict to search."Noneint"‡Returns the last index where <code>sub</code> is found, or raises an error if no such index exists, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive.ˇ

rpartitionb
Y
sep7<a class="anchor" href="../core/string.html">string</a>The string to split on.(tuple"åSplits the input string at the last occurrence of the separator <code>sep</code> and returns the resulting partition as a three-element tuple of the form (before, separator, after). If the input string does not contain the separator, rpartition returns ('', '', self).∑
rsplit»
Y
sep7<a class="anchor" href="../core/string.html">string</a>The string to split on.(
e
maxsplit1<a class="anchor" href="../core/int.html">int</a>The maximum number of splits."unboundlist"·Returns a list of all the words in the string, using <code>sep</code> as the separator, optionally limiting the number of splits to <code>maxsplit</code>. Except for splitting from the right, this method behaves like split().™
rstripù
í
charsM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>4The characters to remove, or all whitespace if None."Nonestring"ˇReturns a copy of the string where trailing characters that appear in <code>chars</code> are removed. Note that <code>chars</code> is not a suffix: all combinations of its value are removed:<pre class="language-python">"abcbaa".rstrip("ab") == "abc"</pre>Ô
split»
Y
sep7<a class="anchor" href="../core/string.html">string</a>The string to split on.(
e
maxsplit1<a class="anchor" href="../core/int.html">int</a>The maximum number of splits."unboundlist"öReturns a list of all the words in the string, using <code>sep</code> as the separator, optionally limiting the number of splits to <code>maxsplit</code>.é

splitlinesñ
â
keepends3<a class="anchor" href="../core/bool.html">bool</a>AWhether the line breaks should be included in the resulting list."Falsesequence"gSplits the string at line boundaries ('\n', '\r\n', '\r') and returns the result as a new mutable list.¿

startswithË

sub≠<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../core/tuple.html">tuple</a> of <a class="anchor" href="../core/string.html">string</a>s7The prefix (or tuple of alternative prefixes) to match.(
u
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code> Test beginning at this position."0
v
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code> Stop comparing at this position."Nonebool"∆Returns True if the string starts with <code>sub</code>, otherwise False, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive.«
stripù
í
charsM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>4The characters to remove, or all whitespace if None."Nonestring"ùReturns a copy of the string where leading or trailing characters that appear in <code>chars</code> are removed. Note that <code>chars</code> is neither a prefix nor a suffix: all combinations of its value are removed:<pre class="language-python">"aabcbcbaa".strip("ab") == "cbc"</pre>ñ
titlestring"ÇConverts the input string into title case, i.e. every word starts with an uppercase letter while the remaining letters are lowercase. In this context, a word means strictly a sequence of letters. This method does not support supplementary Unicode characters.A
upperstring".Returns the upper case version of this string.øA language built-in type to support strings. Examples of string literals:<br><pre class="language-python">a = 'abc\ndef'
b = "ab'cd"
c = """multiline string"""

# Strings support slicing (negative index starts from the end):
x = "hello"[2:4]  # "ll"
y = "hello"[1:-1]  # "ell"
z = "hello"[:4]  # "hell"
# Slice steps can be used, too:
s = "hello"[::2] # "hlo"
t = "hello"[3:0:-1] # "lle"
</pre>Strings are not directly iterable, use the <code>.elems()</code> method to iterate over their characters. Examples:<br><pre class="language-python">"bc" in "abcd"   # evaluates to True
x = [c for c in "abc".elems()]  # x == ["a", "b", "c"]</pre>
Implicit concatenation of strings is not allowed; use the <code>+</code> operator instead. Comparison operators perform a lexicographical comparison; use <code>==</code> to test for equality.
Ê
tuple‹The built-in tuple type. Example tuple expressions:<br><pre class=language-python>x = (1, 2, 3)</pre>Accessing elements is possible using indexing (starts from <code>0</code>):<br><pre class=language-python>e = x[1]   # e == 2</pre>Lists support the <code>+</code> operator to concatenate two tuples. Example:<br><pre class=language-python>x = (1, 2) + (3, 4)   # x == (1, 2, 3, 4)
x = ("a", "b")
x += ("c",)            # x == ("a", "b", "c")</pre>Similar to lists, tuples support slice operations:<pre class=language-python>('a', 'b', 'c', 'd')[1:3]   # ('b', 'c')
('a', 'b', 'c', 'd')[::2]  # ('a', 'c')
('a', 'b', 'c', 'd')[3:0:-1]  # ('d', 'c', 'b')</pre>Tuples are immutable, therefore <code>x[1] = "a"</code> is not supported.Æ
existing_rulee
Z
name7<a class="anchor" href="../core/string.html">string</a>The name of the target.(unknown"≥Returns an immutable dict-like object that describes the attributes of a rule instantiated in this thread's package, or <code>None</code> if no rule instance of that name exists.<p>Here, an <em>immutable dict-like object</em> means a deeply immutable object <code>x</code> supporting dict-like iteration, <code>len(x)</code>, <code>name in x</code>, <code>x[name]</code>, <code>x.get(name)</code>, <code>x.items()</code>, <code>x.keys()</code>, and <code>x.values()</code>.<p>The result contains an entry for each attribute, with the exception of private ones (whose names do not start with a letter) and a few unrepresentable legacy attribute types. In addition, the dict contains entries for the rule instance's <code>name</code> and <code>kind</code> (for example, <code>'cc_binary'</code>).<p>The values of the result represent attribute values as follows:<ul><li>Attributes of type str, int, and bool are represented as is.</li><li>Labels are converted to strings of the form <code>':foo'</code> for targets in the same package or <code>'//pkg:name'</code> for targets in a different package.</li><li>Lists are represented as tuples, and dicts are converted to new, mutable dicts. Their elements are recursively converted in the same fashion.</li><li><code>select</code> values are returned with their contents transformed as described above.</li><li>Attributes for which no value was specified during rule instantiation and whose default value is computed are excluded from the result. (Computed defaults cannot be computed until the analysis phase.).</li></ul><p>If possible, use this function only in <a href="https://bazel.build/extending/macros#finalizers">implementation functions of rule finalizer symbolic macros</a>. Use of this function in other contexts is not recommened, and will be disabled in a future Bazel release; it makes <code>BUILD</code> files brittle and order-dependent. Also, beware that it differs subtly from the two other conversions of rule attribute values from internal form to Starlark: one used by computed defaults, the other used by <code>ctx.attr.foo</code>.(ã
existing_rules	unknown"ÎReturns an immutable dict-like object describing the rules so far instantiated in this thread's package. Each entry of the dict-like object maps the name of the rule instance to the result that would be returned by <code>existing_rule(name)</code>.<p>Here, an <em>immutable dict-like object</em> means a deeply immutable object <code>x</code> supporting dict-like iteration, <code>len(x)</code>, <code>name in x</code>, <code>x[name]</code>, <code>x.get(name)</code>, <code>x.items()</code>, <code>x.keys()</code>, and <code>x.values()</code>.<p>If possible, use this function only in <a href="https://bazel.build/extending/macros#finalizers">implementation functions of rule finalizer symbolic macros</a>. Use of this function in other contexts is not recommened, and will be disabled in a future Bazel release; it makes <code>BUILD</code> files brittle and order-dependent.(„
exports_filesı
õ
srcss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sThe list of files to export.(
ê

visibilityM<a class="anchor" href="../core/list.html">sequence</a>; or <code>None</code>¨A visibility declaration can to be specified. The files will be visible to the targets specified. If no visibility is specified, the files will be visible to every package."None
∑
licensesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>Licenses to be specified."NoneNoneType"XSpecifies a list of files belonging to this package that are exported to other packages.(í	
glob‚
©
includes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s%The list of glob patterns to include."[]
©
excludes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s%The list of glob patterns to exclude."[]
z
exclude_directories1<a class="anchor" href="../core/int.html">int</a>-A flag whether to exclude directories or not."1
Å
allow_emptyËWhether we allow glob patterns to match nothing. If `allow_empty` is False, each individual include pattern must match something and also the final result must be non-empty (after the matches of the `exclude` patterns are excluded)."unboundsequence"¢Glob returns a new, mutable, sorted list of every file in the current package that:<ul>
<li>Matches at least one pattern in <code>include</code>.</li>
<li>Does not match any of the patterns in <code>exclude</code> (default <code>[]</code>).</li></ul>
If the <code>exclude_directories</code> argument is enabled (set to <code>1</code>), files of type directory will be omitted from the results (default <code>1</code>).(˛
module_namestring"‚The name of the Bazel module associated with the repo this package is in. If this package is from a repo defined in WORKSPACE instead of MODULE.bazel, this is empty. For repos generated by module extensions, this is the name of the module hosting the extension. It's the same as the <code>module.name</code> field seen in <code>module_ctx.modules</code>.(ä
module_versionstring"ÎThe version of the Bazel module associated with the repo this package is in. If this package is from a repo defined in WORKSPACE instead of MODULE.bazel, this is empty. For repos generated by module extensions, this is the version of the module hosting the extension. It's the same as the <code>module.version</code> field seen in <code>module_ctx.modules</code>.(Ì
package_default_visibilitylist"ƒReturns the default visibility of the package being evaluated. This is the value of the <code>default_visibility</code> parameter of <code>package()</code>, extended to include the package itself.(Ñ
package_group·
a
name7<a class="anchor" href="../core/string.html">string</a>The unique name for this rule.(
∂
packagess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s1A complete enumeration of packages in this group."[]
∏
includess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s3Other package groups that are included in this one."[]NoneType"åThis function defines a set of packages and assigns a label to the group. The label can be referenced in <code>visibility</code> attributes.(è
package_namestring"ÚThe name of the package being evaluated, without the repository name. For example, in the BUILD file <code>some/package/BUILD</code>, its value will be <code>some/package</code>. If the BUILD file calls a function defined in a .bzl file, <code>package_name()</code> will match the caller BUILD file package. The value will always be an empty string for the root package.(π
package_relative_labelÂ
€
inputu<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>YThe input label string or Label object. If a Label object is passed, it's returned as is.(Label"¥Converts the input string into a <a href='../builtins/Label.html'>Label</a> object, in the context of the package currently being initialized (that is, the <code>BUILD</code> file for which the current macro is executing). If the input is already a <code>Label</code>, it is returned unchanged.<p>This function may only be called while evaluating a BUILD file and the macros it directly or indirectly calls; it may not be called in (for instance) a rule implementation function. <p>The result of this function is the same <code>Label</code> value as would be produced by passing the given string to a label-valued attribute of a target declared in the BUILD file. <p><i>Usage note:</i> The difference between this function and <a href='../builtins/Label.html#Label'>Label()</a></code> is that <code>Label()</code> uses the context of the package of the <code>.bzl</code> file that called it, not the package of the <code>BUILD</code> file. Use <code>Label()</code> when you need to refer to a fixed target that is hardcoded into the macro, such as a compiler. Use <code>package_relative_label()</code> when you need to normalize a label string supplied by the BUILD file to a <code>Label</code> object. (There is no way to convert a string to a <code>Label</code> in the context of a package other than the BUILD file or the calling .bzl file. For that reason, outer macros should always prefer to pass Label objects to inner macros rather than label strings.)(â
	repo_namestring"pThe canonical name of the repository containing the package currently being evaluated, with no leading at-signs.(±
repository_namestring"ë<b>Experimental</b>. This API is experimental and may change at any time. Please do not depend on it. It may be enabled on an experimental basis by setting <code>--+incompatible_enable_deprecated_label_apis</code> <br><strong>Deprecated.</strong> Prefer to use <a href="#repo_name"><code>repo_name</code></a> instead, which doesn't contain the spurious leading at-sign, but behaves identically otherwise.<p>The canonical name of the repository containing the package currently being evaluated, with a single at-sign (<code>@</code>) prefixed. For example, in packages that are called into existence by the WORKSPACE stanza <code>local_repository(name='local', path=...)</code> it will be set to <code>@local</code>. In packages in the main repository, it will be set to <code>@</code>.(ú
subpackages∑
ª
includes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s9The list of glob patterns to include in subpackages scan.(
ø
excludes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s;The list of glob patterns to exclude from subpackages scan."[]
™
allow_empty3<a class="anchor" href="../core/bool.html">bool</a>ﬁWhether we fail if the call returns an empty list. By default empty list indicates potential error in BUILD file where the call to subpackages() is superflous.  Setting to true allows this function to succeed in that case."Falsesequence"–Returns a new mutable list of every direct subpackage of the current package, regardless of file-system directory depth. List returned is sorted and contains the names of subpackages relative to the current package. It is advised to prefer using the methods in bazel_skylib.subpackages module rather than calling this function directly.(
Falsebool
Truebool
NoneNoneType¨
absó
ã
xk<a class="anchor" href="../core/int.html">int</a>; or <a class="anchor" href="../core/float.html">float</a>A number (int or float)(unknown"äReturns the absolute value of a number (a non-negative number with the same magnitude).<pre class="language-python">abs(-2.3) == 2.3</pre>π
all9
1
elementsiterableA collection of elements.(bool"ˆReturns true if all elements evaluate to True or if the collection is empty. Elements are converted to boolean using the <a href="#bool">bool</a> function.<pre class="language-python">all(["hello", 3, True]) == True
all([-1, 0, 1]) == False</pre>†
any9
1
elementsiterableA collection of elements.(bool"›Returns true if at least one element evaluates to True. Elements are converted to boolean using the <a href="#bool">bool</a> function.<pre class="language-python">any([-1, 0, 1]) == True
any([False, 0, ""]) == False</pre>“
bool,
$
xThe variable to convert."Falsebool"õConstructor for the bool type. It returns <code>False</code> if the object is <code>None</code>, <code>False</code>, an empty string (<code>""</code>), the number <code>0</code>, or an empty collection (e.g. <code>()</code>, <code>[]</code>). Otherwise, it returns <code>True</code>.Ê
dictê
U
pairsHA dict, or an iterable whose elements are each of length 2 (key, value)."[]
1
**kwargs!Dictionary of additional entries.(8dict" Creates a <a href="../core/dict.html">dictionary</a> from an optional positional argument and an optional set of keyword arguments. In the case where the same key is given multiple times, the last value will be used. Entries supplied via keyword arguments are considered to come after entries supplied via the positional argument.á
dir#

xThe object to check.(list"[Returns a list of strings: the names of the attributes and methods of the parameter object.¡
	enumeraten

listinput sequence.(
K
start1<a class="anchor" href="../core/int.html">int</a>start index."0list"√Returns a list of pairs (two-element tuples), with the index (int) and the item from the input sequence.
<pre class="language-python">enumerate([24, 21, 84]) == [(0, 24), (1, 21), (2, 84)]</pre>
¬
failê
{
msgnDeprecated: use positional arguments instead. This argument acts like an implicit leading positional argument."None
ª
attrM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>^Deprecated. Causes an optional prefix containing this string to be added to the error message."None
Ü
sep7<a class="anchor" href="../core/string.html">string</a>AThe separator string between the objects, default is space (" ")."" "
í
stack_trace3<a class="anchor" href="../core/bool.html">bool</a>HIf False stack trace is elided from failure for friendlier user messages"True
™
*argsúA list of values, formatted with debugPrint (which is equivalent to str by default) and joined with sep (defaults to " "), that appear in the error message.(0NoneType"'Causes execution to fail with an error.ƒ
floatè
Ö
xﬂ<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../core/int.html">int</a>; or <a class="anchor" href="../core/float.html">float</a>The value to convert."unboundfloat"®Returns x as a float value. <ul><li>If <code>x</code> is already a float, <code>float</code> returns it unchanged. <li>If <code>x</code> is a bool, <code>float</code> returns 1.0 for True and 0.0 for False. <li>If <code>x</code> is an int, <code>float</code> returns the nearest finite floating-point value to x, or an error if the magnitude is too large. <li>If <code>x</code> is a string, it must be a valid floating-point literal, or be equal (ignoring case) to <code>NaN</code>, <code>Inf</code>, or <code>Infinity</code>, optionally preceded by a <code>+</code> or <code>-</code> sign. </ul>Any other value causes an error. With no argument, <code>float()</code> returns 0.0.‹
getattrê
.
x'The struct whose attribute is accessed.(
d
name7<a class="anchor" href="../core/string.html">string</a>!The name of the struct attribute.(
o
default[The default value to return in case the struct doesn't have an attribute of the given name."unboundunknown"ΩReturns the struct's field of the given name if it exists. If not, it either returns <code>default</code> (if specified) or raises an error. <code>getattr(x, "foobar")</code> is equivalent to <code>x.foobar</code>.<pre class="language-python">getattr(ctx.attr, "myattr")
getattr(ctx.attr, "myattr", "mydefault")</pre>—
hasattrÇ

xThe object to check.(
]
name7<a class="anchor" href="../core/string.html">string</a>The name of the attribute.(bool"¿Returns True if the object <code>x</code> has an attribute or method of the given <code>name</code>, otherwise False. Example:<br><pre class="language-python">hasattr(ctx.attr, "myattr")</pre>ê
hash`
Y
value7<a class="anchor" href="../core/string.html">string</a>String value to hash.(int"•Return a hash value for a string. This is computed deterministically using the same algorithm as Java's <code>String.hashCode()</code>, namely: <pre class="language-python">s[0] * (31^(n-1)) + s[1] * (31^(n-2)) + ... + s[n-1]</pre> Hashing of values besides strings is not currently supported.∫
int∑
ˇ
xﬂ<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../core/int.html">int</a>; or <a class="anchor" href="../core/float.html">float</a>The string to convert.(
≠
base1<a class="anchor" href="../core/int.html">int</a>ËThe base used to interpret a string value; defaults to 10. Must be between 2 and 36 (inclusive), or 0 to detect the base as if <code>x</code> were an integer literal. This parameter must not be supplied if the value is not a string."unboundint"¯Returns x as an int value.<ul><li>If <code>x</code> is already an int, <code>int</code> returns it unchanged.<li>If <code>x</code> is a bool, <code>int</code> returns 1 for True and 0 for False.<li>If <code>x</code> is a string, it must have the format     <code>&lt;sign&gt;&lt;prefix&gt;&lt;digits&gt;</code>.     <code>&lt;sign&gt;</code> is either <code>"+"</code>, <code>"-"</code>,     or empty (interpreted as positive). <code>&lt;digits&gt;</code> are a     sequence of digits from 0 up to <code>base</code> - 1, where the letters a-z     (or equivalently, A-Z) are used as digits for 10-35. In the case where     <code>base</code> is 2/8/16, <code>&lt;prefix&gt;</code> is optional and may     be 0b/0o/0x (or equivalently, 0B/0O/0X) respectively; if the     <code>base</code> is any other value besides these bases or the special value     0, the prefix must be empty. In the case where <code>base</code> is 0, the     string is interpreted as an integer literal, in the sense that one of the     bases 2/8/10/16 is chosen depending on which prefix if any is used. If     <code>base</code> is 0, no prefix is used, and there is more than one digit,     the leading digit cannot be 0; this is to avoid confusion between octal and     decimal. The magnitude of the number represented by the string must be within     the allowed range for the int type.<li>If <code>x</code> is a float, <code>int</code> returns the integer value of    the float, rounding towards zero. It is an error if x is non-finite (NaN or    infinity).</ul>This function fails if <code>x</code> is any other type, or if the value is a string not satisfying the above format. Unlike Python's <code>int</code> function, this function does not allow zero arguments, and does not allow extraneous whitespace for string arguments.<p>Examples:<pre class="language-python">int("123") == 123
int("-123") == -123
int("+123") == 123
int("FF", 16) == 255
int("0xFF", 16) == 255
int("10", 0) == 10
int("-0x10", 0) == -16
int("-0x10", 0) == -16
int("123.456") == 123
</pre>ﬂ
lenu
n
xDiterable; or <a class="anchor" href="../core/string.html">string</a>!The value whose length to report.(int"aReturns the length of a string, sequence (such as a list or tuple), dict, set, or other iterable.Ñ
list1
)
xiterableThe object to convert."[]list"»Returns a new list with the same elements as the given iterable value.<pre class="language-python">list([1, 2]) == [1, 2]
list((2, 3, 2)) == [2, 3, 2]
list({5: "a", 2: "b", 4: "c"}) == [5, 2, 4]</pre>Ú
max°
l
keycallable; or <code>None</code>?An optional function applied to each element before comparison."None
(
*argsThe elements to be checked.(0unknown"∆Returns the largest one of all given arguments. If only one positional argument is provided, it must be a non-empty iterable.It is an error if elements are not comparable (for example int with string), or if no arguments are given.<pre class="language-python">
max(2, 5, 4) == 5
max([5, 6, 3]) == 6
max("two", "three", "four", key = len) =="three"  # the longest
max([1, -1, -2, 2], key = abs) == -2  # the first encountered with maximal key value
</pre>Ù
min°
l
keycallable; or <code>None</code>?An optional function applied to each element before comparison."None
(
*argsThe elements to be checked.(0unknown"»Returns the smallest one of all given arguments. If only one positional argument is provided, it must be a non-empty iterable. It is an error if elements are not comparable (for example int with string), or if no arguments are given.<pre class="language-python">
min(2, 5, 4) == 2
min([5, 6, 3]) == 3
min("six", "three", "four", key = len) == "six"  # the shortest
min([2, -2, -1, 1], key = abs) == -1  # the first encountered with minimal key value
</pre>∫
print∑
Ü
sep7<a class="anchor" href="../core/string.html">string</a>AThe separator string between the objects, default is space (" ")."" "
"
*argsThe objects to print.(0NoneType"ˆPrints <code>args</code> as debug output. It will be prefixed with the string <code>"DEBUG"</code> and the location (file and line number) of this call. The exact way in which the arguments are converted to strings is unspecified and may change at any time. In particular, it may be different from (and more detailed than) the formatting done by <a href='#str'><code>str()</code></a> and <a href='#repr'><code>repr()</code></a>.<p>Using <code>print</code> in production code is discouraged due to the spam it creates for users. For deprecations, prefer a hard error using <a href="#fail"><code>fail()</code></a> whenever possible.º
rangeÄ
ß
start_or_stop1<a class="anchor" href="../core/int.html">int</a>aValue of the start element if stop is provided, otherwise value of stop and the actual start is 0(
ÿ
stop1<a class="anchor" href="../core/int.html">int</a>ìoptional index of the first item <i>not</i> to be included in the resulting list; generation of the list stops before <code>stop</code> is reached."unbound
o
step1<a class="anchor" href="../core/int.html">int</a>1The increment (default is 1). It may be negative."1sequence"ØCreates a list where items go from <code>start</code> to <code>stop</code>, using a <code>step</code> increment. If a single argument is provided, items will range from 0 to that element.<pre class="language-python">range(4) == [0, 1, 2, 3]
range(3, 9, 2) == [3, 5, 7]
range(3, 0, -1) == [3, 2, 1]</pre>∫
repr'

xThe object to convert.(string"àConverts any object to a string representation. This is useful for debugging.<br><pre class="language-python">repr("ab") == '"ab"'</pre>è
reversedQ
I
sequenceiterable1The iterable sequence (e.g. list) to be reversed.(list"ØReturns a new, unfrozen list that contains the elements of the original iterable sequence in reversed order.<pre class="language-python">reversed([3, 5, 4]) == [4, 5, 3]</pre>å
set@
9
elementsiterableAn iterable of hashable values."[]set"¬Creates a new <a href="../core/set.html">set</a> containing the unique elements of a given
iterable, preserving iteration order.

<p>If called with no argument, <code>set()</code> returns a new empty set.

<p>For example,
<pre class=language-python>
set()                          # an empty set
set([3, 1, 1, 2])              # set([3, 1, 2]), a set of three elements
set({"k1": "v1", "k2": "v2"})  # set(["k1", "k2"]), a set of two elements
</pre>
Ï
sortedò
6
iterableiterableThe iterable sequence to sort.(
l
keycallable; or <code>None</code>?An optional function applied to each element before comparison."None
j
reverse3<a class="anchor" href="../core/bool.html">bool</a>#Return results in descending order."Falselist"∆Returns a new sorted list containing all the elements of the supplied iterable sequence. An error may occur if any pair of elements x, y may not be compared using x < y. The elements are sorted into ascending order, unless the reverse argument is True, in which case the order is descending.
 Sorting is stable: elements that compare equal retain their original relative order.
<pre class="language-python">
sorted([3, 5, 4]) == [3, 4, 5]
sorted([3, 5, 4], reverse = True) == [5, 4, 3]
sorted(["two", "three", "four"], key = len) == ["two", "four", "three"]  # sort by length
</pre>Æ
str'

xThe object to convert.(string"~Converts any object to string. This is useful for debugging.<pre class="language-python">str("ab") == "ab"
str(8) == "8"</pre>Ü
tuple2
)
xiterableThe object to convert."()tuple"»Returns a tuple with the same elements as the given iterable value.<pre class="language-python">tuple([1, 2]) == (1, 2)
tuple((2, 3, 2)) == (2, 3, 2)
tuple({5: "a", 2: "b", 4: "c"}) == (5, 2, 4)</pre>–
type-
#
xThe object to check type of.(string"òReturns the type name of its argument. This is useful for debugging and type-checking. Examples:<pre class="language-python">type(2) == "int"
type([1]) == "list"
type(struct(a = 2)) == "struct"</pre>This function might change in the future. To write Python-compatible code and be future-proof, use it only to compare return values: <pre class="language-python">if type(x) == type([]):  # if x is a list</pre>ç
zip"

*argslists to zip.(0list"·Returns a <code>list</code> of <code>tuple</code>s, where the i-th tuple contains the i-th element from each of the argument sequences or iterables. The list has the size of the shortest input. With a single iterable argument, it returns a list of 1-tuples. With no arguments, it returns an empty list. Examples:<pre class="language-python">zip()  # == []
zip([1, 2])  # == [(1,), (2,)]
zip([1, 2], [3, 4])  # == [(1, 3), (2, 4)]
zip([1, 2], [3, 4, 5])  # == [(1, 3), (2, 4)]</pre>æ
nativenative"©A built-in module to support native rules and other package helper functions. All native rules appear as functions in this module, e.g. <code>native.cc_library</code>. Note that the native module is only available in the loading phase (i.e. for macros, not for rule implementations). Attributes will ignore <code>None</code> values, and treat them as if the attribute was unset.<br>The following functions are also available:(Ë
depsetŒ
ç
directM<a class="anchor" href="../core/list.html">sequence</a>; or <code>None</code>.A list of <i>direct</i> elements of a depset. "None
ø
order7<a class="anchor" href="../core/string.html">string</a>rThe traversal strategy for the new depset. See <a href="../builtins/depset.html">here</a> for the possible values."	"default"
Ò

transitiveç<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/depset.html">depset</a>s; or <code>None</code>MA list of depsets whose elements will become indirect elements of the depset."Nonedepset"äCreates a <a href="../builtins/depset.html">depset</a>. The <code>direct</code> parameter is a list of direct elements of the depset, and <code>transitive</code> parameter is a list of depsets whose elements become indirect elements of the created depset. The order in which elements are returned when the depset is converted to a list is specified by the <code>order</code> parameter. See the <a href="https://bazel.build/extending/depsets">Depsets overview</a> for more information.
<p>All elements (direct and indirect) of a depset must be of the same type, as obtained by the expression <a href="../globals/all#type"><code>type(x)</code></a>.
<p>Because a hash-based set is used to eliminate duplicates during iteration, all elements of a depset should be hashable. However, this invariant is not currently checked consistently in all constructors. Use the --incompatible_always_check_depset_elements flag to enable consistent checking; this will be the default behavior in future releases;  see <a href='https://github.com/bazelbuild/bazel/issues/10313'>Issue 10313</a>.
<p>In addition, elements must currently be immutable, though this restriction will be relaxed in future.
<p> The order of the created depset should be <i>compatible</i> with the order of its <code>transitive</code> depsets. <code>"default"</code> order is compatible with any other order, all other orders are only compatible with themselves.(K
jsonjson";Module json is a Starlark module of JSON-related functions.(;
protoproto")A module for protocol message processing.(÷
select¯
‚
x3<a class="anchor" href="../core/dict.html">dict</a>•A dict that maps configuration conditions to values. Each key is a <a href="../builtins/Label.html">Label</a> or a label string that identifies a config_setting or constraint_value instance. See the <a href="https://bazel.build/extending/legacy-macros#label-resolution">documentation on macros</a> for when to use a Label instead of a string. If <code>--incompatible_resolve_select_keys_eagerly</code> is enabled, the keys are resolved to <code>Label</code> objects relative to the package of the file that contains this call to <code>select</code>.(
á
no_match_error7<a class="anchor" href="../core/string.html">string</a>8Optional custom error to report if no condition matches."''unknown"Œ<code>select()</code> is the helper function that makes a rule attribute <a href="#configurable-attributes">configurable</a>. See <a href="/reference/be/functions#select">build encyclopedia</a> for details.(¡
configuration_field´
ê
fragment7<a class="anchor" href="../core/string.html">string</a>IThe name of a configuration fragment which contains the late-bound value.(
É
name7<a class="anchor" href="../core/string.html">string</a>@The name of the value to obtain from the configuration fragment.(LateBoundDefault"˘References a late-bound default value for an attribute of type <a href="../toplevel/attr.html#label">label</a>. A value is 'late-bound' if it requires the configuration to be built before determining the value. Any attribute using this as a value must <a href="https://bazel.build/extending/rules#private-attributes">be private</a>. <p>Example usage: <p>Defining a rule attribute: <br><pre class=language-python>'_foo': attr.label(default=configuration_field(fragment='java', name='toolchain'))</pre><p>Accessing in rule implementation: <br><pre class=language-python>  def _rule_impl(ctx):
    foo_info = ctx.attr._foo
    ...</pre>(ﬂ

visibility˘

Ï

value‡
A list of package specification strings, or a single package specification string.<p>Package specifications follow the same format as for <code><a href='/reference/be/functions#package_group'>package_group</a></code>, except that negative package specifications are not permitted. That is, a specification may have the forms:<ul><li><code>"//foo"</code>: the package <code>//foo</code><li><code>"//foo/..."</code>: the package <code>//foo</code> and all of its subpackages.<li><code>"public"</code> or <code>"private"</code>: all packages or no packages, respectively</ul><p>The "@" syntax is not allowed; all specifications are interpreted relative to the current module's repository.<p>If <code>value</code> is a list of strings, the set of packages granted visibility to this module is the union of the packages represented by each specification. (An empty list has the same effect as <code>private</code>.) If <code>value</code> is a single string, it is treated as if it were the singleton list <code>[value]</code>.<p>Note that the flags <code>--incompatible_package_group_has_public_syntax</code> and <code>--incompatible_fix_package_group_reporoot_syntax</code> have no effect on this argument. The <code>"public"</code> and <code>"private"</code> values are always available, and <code>"//..."</code> is always interpreted as "all packages in the current repository".(NoneType"“<p>Sets the load visibility of the .bzl module currently being initialized.<p>The load visibility of a module governs whether or not other BUILD and .bzl files may load it. (This is distinct from the target visibility of the underlying .bzl source file, which governs whether the file may appear as a dependency of other targets.) Load visibility works at the level of packages: To load a module the file doing the loading must live in a package that has been granted visibility to the module. A module can always be loaded within its own package, regardless of its visibility.<p><code>visibility()</code> may only be called once per .bzl file, and only at the top level, not inside a function. The preferred style is to put this call immediately below the <code>load()</code> statements and any brief logic needed to determine the argument.<p>If the flag <code>--check_bzl_visibility</code> is set to false, load visibility violations will emit warnings but not fail the build.(ÄE
aspect∞C
à
implementation;<a class="anchor" href="../core/function.html">function</a>∂A Starlark function that implements this aspect, with exactly two parameters: <a href="../builtins/Target.html">Target</a> (the target to which the aspect is applied) and <a href="../builtins/ctx.html">ctx</a> (the rule context which the target is created from). Attributes of the target are available via the <code>ctx.rule</code> field. This function is evaluated during the analysis phase for each application of an aspect to a target.(
…
attr_aspects≥<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <a class="anchor" href="../core/function.html">function</a>˛Accepts a list of attribute names or [Experimental] a function that returns the list of attribute names. The aspect propagates along dependencies specified in the attributes of a target with these names. Common values here include <code>deps</code> and <code>exports</code>. The list can also contain a single string <code>"*"</code> to propagate along all dependencies of a target."[]
 
toolchains_aspectsw<a class="anchor" href="../core/list.html">sequence</a>; or <a class="anchor" href="../core/function.html">function</a>∂Accepts a list of toolchain types or [Experimental] a function that returns the list of toolchain types. The aspect propagates to target toolchains which match these toolchain types."[]
É
attrs3<a class="anchor" href="../core/dict.html">dict</a>¿A dictionary declaring all the attributes of the aspect. It maps from an attribute name to an attribute object, like <code>attr.label</code> or <code>attr.string</code> (see <a href="../toplevel/attr.html"><code>attr</code></a> module). Aspect attributes are available to implementation function as fields of <code>ctx</code> parameter. <p>Implicit attributes starting with <code>_</code> must have default values, and have type <code>label</code> or <code>label_list</code>.</p> <p>Explicit attributes must have type <code>string</code>, and must use the <code>values</code> restriction. Explicit attributes restrict the aspect to only be used with rules that have attributes of the same name, type, and valid values according to the restriction.</p>
<p>Declared attributes will convert <code>None</code> to the default value.</p>
"{}
ƒ	
required_providers7<a class="anchor" href="../core/list.html">sequence</a>This attribute allows the aspect to limit its propagation to only the targets whose rules advertise its required providers. The value must be a list containing either individual providers or lists of providers but not both. For example, <code>[[FooInfo], [BarInfo], [BazInfo, QuxInfo]]</code> is a valid value while <code>[FooInfo, BarInfo, [BazInfo, QuxInfo]]</code> is not valid.<p>An unnested list of providers will automatically be converted to a list containing one list of providers. That is, <code>[FooInfo, BarInfo]</code> will automatically be converted to <code>[[FooInfo, BarInfo]]</code>.<p>To make some rule (e.g. <code>some_rule</code>) targets visible to an aspect, <code>some_rule</code> must advertise all providers from at least one of the required providers lists. For example, if the <code>required_providers</code> of an aspect are <code>[[FooInfo], [BarInfo], [BazInfo, QuxInfo]]</code>, this aspect can see <code>some_rule</code> targets if and only if <code>some_rule</code> provides <code>FooInfo</code>, <em>or</em> <code>BarInfo</code>, <em>or</em> both <code>BazInfo</code> <em>and</em> <code>QuxInfo</code>."[]
«
required_aspect_providers7<a class="anchor" href="../core/list.html">sequence</a>ÏThis attribute allows this aspect to inspect other aspects. The value must be a list containing either individual providers or lists of providers but not both. For example, <code>[[FooInfo], [BarInfo], [BazInfo, QuxInfo]]</code> is a valid value while <code>[FooInfo, BarInfo, [BazInfo, QuxInfo]]</code> is not valid.<p>An unnested list of providers will automatically be converted to a list containing one list of providers. That is, <code>[FooInfo, BarInfo]</code> will automatically be converted to <code>[[FooInfo, BarInfo]]</code>. <p>To make another aspect (e.g. <code>other_aspect</code>) visible to this aspect, <code>other_aspect</code> must provide all providers from at least one of the lists. In the example of <code>[[FooInfo], [BarInfo], [BazInfo, QuxInfo]]</code>, this aspect can see <code>other_aspect</code> if and only if <code>other_aspect</code> provides <code>FooInfo</code>, <em>or</em> <code>BarInfo</code>, <em>or</em> both <code>BazInfo</code> <em>and</em> <code>QuxInfo</code>."[]
·
provides7<a class="anchor" href="../core/list.html">sequence</a>óA list of providers that the implementation function must return.<p>It is an error if the implementation function omits any of the types of providers listed here from its return value. However, the implementation function may return additional providers not listed here.<p>Each element of the list is an <code>*Info</code> object returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a>. When a target of the rule is used as a dependency for a target that declares a required provider, it is not necessary to specify that provider here. It is enough that the implementation function returns it. However, it is considered best practice to specify it, even though this is not required. The <a href='../globals/bzl.html#aspect.required_providers'><code>required_providers</code></a> field of an <a href='../globals/bzl.html#aspect'>aspect</a> does, however, require that providers are specified here."[]
∆
requiresw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Aspect.html">Aspect</a>s=List of aspects required to be propagated before this aspect."[]
Á
propagation_predicateQ<a class="anchor" href="../core/function.html">function</a>; or <code>None</code>uExperimental: a function that returns a boolean value indicating whether the aspect should be propagated to a target."None
‡
	fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sZList of names of configuration fragments that the aspect requires in target configuration."[]
„
host_fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sXList of names of configuration fragments that the aspect requires in host configuration."[]
·

toolchains7<a class="anchor" href="../core/list.html">sequence</a>ïIf set, the set of toolchains this aspect requires. The list can contain String, Label, or StarlarkToolchainTypeApi objects, in any combination. Toolchains will be found by checking the current platform, and provided to the aspect implementation via <code>ctx.toolchain</code>."[]
∞
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>TA description of the aspect that can be extracted by documentation generating tools."None
π
apply_to_generating_rules3<a class="anchor" href="../core/bool.html">bool</a>ﬂIf true, the aspect will, when applied to an output file, instead apply to the output file's generating rule. <p>For example, suppose an aspect propagates transitively through attribute `deps` and it is applied to target `alpha`. Suppose `alpha` has `deps = [':beta_output']`, where `beta_output` is a declared output of a target `beta`. Suppose `beta` has a target `charlie` as one of its `deps`. If `apply_to_generating_rules=True` for the aspect, then the aspect will propagate through `alpha`, `beta`, and `charlie`. If False, then the aspect will propagate only to `alpha`. </p><p>False by default.</p>"False
Ï
exec_compatible_withs<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s[A list of constraints on the execution platform that apply to all instances of this aspect."[]
á
exec_groupsI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>¶Dict of execution group name (string) to <a href='../globals/bzl.html#exec_group'><code>exec_group</code>s</a>. If set, allows aspects to run actions on multiple execution platforms within a single instance. See <a href='/reference/exec-groups'>execution groups documentation</a> for more info."None
æ
subrulesy<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Subrule.html">Subrule</a>s3Experimental: list of subrules used by this aspect."[]Aspect"¿Creates a new aspect. The result of this function must be stored in a global value. Please see the <a href="https://bazel.build/extending/aspects">introduction to Aspects</a> for more details.(Â

exec_group∞
›

toolchains7<a class="anchor" href="../core/list.html">sequence</a>ëThe set of toolchains this execution group requires. The list can contain String, Label, or StarlarkToolchainTypeApi objects, in any combination."[]
¡
exec_compatible_withs<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s0A list of constraints on the execution platform."[]
exec_group"°Creates an <a href='/reference/exec-groups'>execution group</a> which can be used to create actions for a specific execution platform during rule implementation.(‡
Label
	
input("«Converts a label string into a <code>Label</code> object, in the context of the package where the calling <code>.bzl</code> source file lives. If the given value is already a <code>Label</code>, it is returned unchanged.<p>For macros, a related function, <code><a href='../toplevel/native.html#package_relative_label'>native.package_relative_label()</a></code>, converts the input into a <code>Label</code> in the context of the package currently being constructed. Use that function to mimic the string-to-label conversion that is automatically done by label-valued rule attributes.(«O
macroåL
»
implementation;<a class="anchor" href="../core/function.html">function</a>ˆThe Starlark function implementing this macro. The values of the macro's attributes are passed to
the implementation function as keyword arguments. The implementation function must have at least two
named parameters, <code>name</code> and <code>visibility</code>, and if the macro inherits
attributes (see <code>inherit_attrs</code> below), it must have a <code>**kwargs</code> residual
keyword parameter.

<p>By convention, the implementation function should have a named parameter for any attribute that
the macro needs to examine, modify, or pass to non-"main" targets, while the "bulk" inherited
attributes which will be passed to the "main" target unchanged are passed as <code>**kwargs</code>.

<p>The implementation function must not return a value. Instead, the implementation function
<em>declares targets</em> by calling rule or macro symbols.

<p>The name of any target or inner symbolic macro declared by a symbolic macro (including by any
Starlark function that the macro's implementation function transitively calls) must either equal
<code>name</code> (this is referred to as the "main" target) or start with <code>name</code>,
followed by a separator chracter (<code>"_"</code>, <code>"-"</code>, or <code>"."</code>) and a
string suffix. (Targets violating this naming scheme are allowed to be declared, but cannot be
built, configured, or depended upon.)

<p>By default, targets declared by a symbolic macro (including by any Starlark function that the
macro's implementation function transitively calls) are visible only in the package containing the
.bzl file defining the macro. To declare targets visible externally, <em>including to the caller of
the symbolic macro</em>, the implementation function must set <code>visibility</code> appropriately
&ndash; typically, by passing <code>visibility = visibility</code> to the rule or macro symbol being
called.

<p>The following APIs are unavailable within a macro implementation function and any Starlark
function it transitively calls:
<ul>
  <li><a href="/reference/be/functions#package"><code>package()</code>, <code>licenses()</code>
  <li><code>environment_group()</code>
  <li><a href="../toplevel/native#glob"><code>native.glob()</code></a> &ndash; instead, you may pass
    a glob into the macro via a label list attribute
  <li><a href="../toplevel/native#subpackages"><code>native.subpackages()</code></a>
  <li>(allowed in rule finalizers only, see <code>finalizer</code> below)
    <a href="../toplevel/native#existing_rules"><code>native.existing_rules()</code></a>,
    <a href="../toplevel/native#existing_rule"><code>native.existing_rule()</code></a>
  <li>(for <code>WORKSPACE</code> threads)
    <a href="../globals/workspace#workspace"><code>workspace()</code></a>,
    <a href="../globals/workspace#register_toolchains"><code>register_toolchains()</code></a>,
    <a href="../globals/workspace#register_execution_platforms><code>register_execution_platforms()</code></a>,
    <a href="../globals/workspace#bind"><code>bind()</code></a>, repository rule instantiation
</ul>
(
∆
attrs3<a class="anchor" href="../core/dict.html">dict</a>ÉA dictionary of the attributes this macro supports, analogous to
<a href="#rule.attrs">rule.attrs</a>. Keys are attribute names, and values are either attribute
objects like <code>attr.label_list(...)</code> (see the <a href="../toplevel/attr.html">attr</a>
module), or <code>None</code>. A <code>None</code> entry means that the macro does not have an
attribute by that name, even if it would have otherwise inherited one via <code>inherit_attrs</code>
(see below).

<p>The special <code>name</code> attribute is predeclared and must not be included in the
dictionary. The <code>visibility</code> attribute name is reserved and must not be included in the
dictionary.

<p>Attributes whose names start with <code>_</code> are private -- they cannot be passed at the call
site of the rule. Such attributes can be assigned a default value (as in
<code>attr.label(default="//pkg:foo")</code>) to create an implicit dependency on a label.

<p>To limit memory usage, there is a cap on the number of attributes that may be declared.
"{}
≤#
inherit_attrs«<a class="anchor" href="../builtins/rule.html">rule</a>; or <a class="anchor" href="../builtins/macro.html">macro</a>; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>–!A rule symbol, macro symbol, or the name of a built-in common attribute list (see below) from which
the macro should inherit attributes.

<p>If <code>inherit_attrs</code> is set to the string <code>"common"</code>, the macro will inherit
<a href="/reference/be/common-definitions#common-attributes">common rule attribute definitions</a>
used by all Starlark rules.

<p>Note that if the return value of <code>rule()</code> or <code>macro()</code> was not assigned to
a global variable in a .bzl file, then such a value has not been registered as a rule or macro
symbol, and therefore cannot be used for <code>inherit_attrs</code>.

<p>The inheritance mechanism works as follows:</p>
<ol>
  <li>The special <code>name</code> and <code>visibility</code> attributes are never inherited;
  <li>Hidden attributes (ones whose name starts with <code>"_"</code>) are never inherited;
  <li>Attributes whose names are already defined in the <code>attrs</code> dictionary are never
    inherited (the entry in <code>attrs</code> takes precedence; note that an entry may be set to
    <code>None</code> to ensure that no attribute by that name gets defined on the macro);
  <li>All other attributes are inherited from the rule or macro and effectively merged into the
    <code>attrs</code> dict.
</ol>

<p>When a non-mandatory attribute is inherited, the default value of the attribute is overridden
to be <code>None</code>, regardless of what it was specified in the original rule or macro. This
ensures that when the macro forwards the attribute's value to an instance of the wrapped rule or
macro &ndash; such as by passing in the unmodified <code>**kwargs</code> &ndash; a value that was
absent from the outer macro's call will also be absent in the inner rule or macro's call (since
passing <code>None</code> to an attribute is treated the same as omitting the attribute).
This is important because omitting an attribute has subtly different semantics from passing
its apparent default value. In particular, omitted attributes are not shown in some <code>bazel
query</code> output formats, and computed defaults only execute when the value is omitted. If the
macro needs to examine or modify an inherited attribute &ndash; for example, to add a value to an
inherited <code>tags</code> attribute &ndash; you must make sure to handle the <code>None</code>
case in the macro's implementation function.

<p>For example, the following macro inherits all attributes from <code>native.cc_library</code>,
except for <code>cxxopts</code> (which is removed from the attribute list) and <code>copts</code>
(which is given a new definition). It also takes care to checks for the default <code>None</code>
value of the inherited <code>tags</code> attribute before appending an additional tag.

<pre class="language-python">
def _my_cc_library_impl(name, visibility, tags, **kwargs):
    # Append a tag; tags attr was inherited from native.cc_library, and
    # therefore is None unless explicitly set by the caller of my_cc_library()
    my_tags = (tags or []) + ["my_custom_tag"]
    native.cc_library(
        name = name,
        visibility = visibility,
        tags = my_tags,
        **kwargs
    )

my_cc_library = macro(
    implementation = _my_cc_library_impl,
    inherit_attrs = native.cc_library,
    attrs = {
        "cxxopts": None,
        "copts": attr.string_list(default = ["-D_FOO"]),
    },
)
</pre>

<p>If <code>inherit_attrs</code> is set, the macro's implementation function <em>must</em> have a
<code>**kwargs</code> residual keyword parameter.

<p>By convention, a macro should pass inherited, non-overridden attributes unchanged to the "main"
rule or macro symbol which the macro is wrapping. Typically, most inherited attributes will not have
a parameter in the implementation function's parameter list, and will simply be passed via
<code>**kwargs</code>. It can be convenient for the implementation function to have explicit
parameters for some inherited attributes (most commonly, <code>tags</code> and
<code>testonly</code>) if the macro needs to pass those attributes to both "main" and non-"main"
targets &ndash; but if the macro also needs to examine or manipulate those attributes, you must take
care to handle the <code>None</code> default value of non-mandatory inherited attributes.
"None
á
	finalizer3<a class="anchor" href="../core/bool.html">bool</a>ΩWhether this macro is a rule finalizer, which is a macro that, regardless of its position in a
<code>BUILD</code> file, is evaluated at the end of package loading, after all non-finalizer targets
have been defined.

<p>Unlike ordinary symbolic macros, rule finalizers may call
<a href="../toplevel/native#existing_rule"><code>native.existing_rule()</code></a> and
<a href="../toplevel/native#existing_rules"><code>native.existing_rules()</code></a> to query the
set of <em>non-finalizer</em> rule targets defined in the current package. Note that
<code>native.existing_rule()</code> and <code>native.existing_rules()</code> cannot access the
targets defined by any rule finalizer, including this one.
"False
Ø
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>SA description of the macro that can be extracted by documentation generating tools."Nonemacro"¨Defines a symbolic macro, which may be called in <code>BUILD</code> files or macros (legacy or
symbolic) to define targets &ndash; possibly multiple ones.

<p>The value returned by <code>macro(...)</code> must be assigned to a global variable in a .bzl
file; the name of the global variable will be the macro symbol's name.

<p>See <a href="/extending/macros">Macros</a> for a comprehensive guide on how to use symbolic
macros.
(æ
materializer_rule€
ı
implementation;<a class="anchor" href="../core/function.html">function</a>£The Starlark function implementing this materializer rule. It must have exactly one parameter: <a href="../builtins/ctx.html">ctx</a>. This function is called during the analysis phase for each instance of the rule. Materializer rules return exactly one and only one MaterializedDepsInfo provider which specifies the dependencies to materialize in place of any instance of this rule in the attributes of another target.(
∫
attrs3<a class="anchor" href="../core/dict.html">dict</a>˜A dictionary to declare all the attributes of the rule. It maps from an attribute name to an attribute object (see
<a href="../toplevel/attr.html"><code>attr</code></a> module). Attributes starting with <code>_</code> are private, and can be used to add an implicit dependency on a label. The attribute <code>name</code> is implicitly added and must not be specified. Attributes <code>visibility</code>, <code>deprecation</code>, <code>tags</code>, <code>testonly</code>, and <code>features</code> are implicitly added and cannot be overridden. Most rules need only a handful of attributes. To limit memory usage, there is a cap on the number of attributes that may be declared.
<p>Declared attributes will convert <code>None</code> to the default value.</p>
"{}
Æ
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>RA description of the rule that can be extracted by documentation generating tools."None
Ë
allow_real_deps3<a class="anchor" href="../core/bool.html">bool</a>òWhether to allow instances of this materializer rule to have real dependencies (non-dormant deps / non-for_dependency_resolution). Subject to allowlist."Falsecallable"»Creates a new materializer rule, which can be called from a BUILD file or a macro to create materializer targets.<p>Materializer targets are used to dynamically select dependencies at analysis time. Targets which depend on a materializer target will see the materialized dependencies, rather than the materializer target itself.(Ê*
provideræ!
≤
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>VA description of the provider that can be extracted by documentation generating tools."None
ÿ
fields¡<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>ÉIf specified, restricts the set of allowed fields. <br>Possible values are:<ul>  <li> list of fields:<br>       <pre class="language-python">provider(fields = ['a', 'b'])</pre><p>  <li> dictionary field name -> documentation:<br>       <pre class="language-python">provider(
       fields = { 'a' : 'Documentation for a', 'b' : 'Documentation for b' })</pre></ul>All fields are optional."None
¢
initcallable; or <code>None</code>ÛAn optional callback for preprocessing and validating the provider's field values during instantiation. If <code>init</code> is specified, <code>provider()</code> returns a tuple of 2 elements: the normal provider symbol and a <em>raw constructor</em>.<p>A precise description follows; see <a href='https://bazel.build/extending/rules#custom_initialization_of_providers'>Rules (Custom initialization of providers)</a> for an intuitive discussion and use cases.<p>Let <code>P</code> be the provider symbol created by calling <code>provider()</code>. Conceptually, an instance of <code>P</code> is generated by calling a default constructor function <code>c(*args, **kwargs)</code>, which does the following:<ul><li>If <code>args</code> is non-empty, an error occurs.</li><li>If the <code>fields</code> parameter was specified when <code>provider()</code> was called, and if <code>kwargs</code> contains any key that was not listed in <code>fields</code>, an error occurs.</li><li>Otherwise, <code>c</code> returns a new instance that has, for each <code>k: v</code> entry in <code>kwargs</code>, a field named <code>k</code> with value <code>v</code>.</ul>In the case where an <code>init</code> callback is <em>not</em> given, a call to the symbol <code>P</code> itself acts as a call to the default constructor function <code>c</code>; in other words, <code>P(*args, **kwargs)</code> returns <code>c(*args, **kwargs)</code>. For example,<pre class="language-python">MyInfo = provider()
m = MyInfo(foo = 1)</pre>will straightforwardly make it so that <code>m</code> is a <code>MyInfo</code> instance with <code>m.foo == 1</code>.<p>But in the case where <code>init</code> is specified, the call <code>P(*args, **kwargs)</code> will perform the following steps instead:<ol><li>The callback is invoked as <code>init(*args, **kwargs)</code>, that is, with the exact same positional and keyword arguments as were passed to <code>P</code>.</li><li>The return value of <code>init</code> is expected to be a dictionary, <code>d</code>, whose keys are field name strings. If it is not, an error occurs.</li><li>A new instance of <code>P</code> is generated as if by calling the default constructor with <code>d</code>'s entries as keyword arguments, as in <code>c(**d)</code>.</li></ol><p>NB: the above steps imply that an error occurs if <code>*args</code> or <code>**kwargs</code> does not match <code>init</code>'s signature, or the evaluation of <code>init</code>'s body fails (perhaps intentionally via a call to <a href="../globals/all.html#fail"><code>fail()</code></a>), or if the return value of <code>init</code> is not a dictionary with the expected schema.<p>In this way, the <code>init</code> callback generalizes normal provider construction by allowing positional arguments and arbitrary logic for preprocessing and validation. It does <em>not</em> enable circumventing the list of allowed <code>fields</code>.<p>When <code>init</code> is specified, the return value of <code>provider()</code> becomes a tuple <code>(P, r)</code>, where <code>r</code> is the <em>raw constructor</em>. In fact, the behavior of <code>r</code> is exactly that of the default constructor function <code>c</code> discussed above. Typically, <code>r</code> is bound to a variable whose name is prefixed with an underscore, so that only the current .bzl file has direct access to it:<pre class="language-python">MyInfo, _new_myinfo = provider(init = ...)</pre>"Noneunknown"ñ	Defines a provider symbol. The resulting value of this function must be stored in a global value to be usable in a rule or aspect implementation. Providers can be instantiated by calling the resulting value as a function, or used directly as an index key for retrieving an instance of that provider from a target. Example:<br><pre class="language-python">MyInfo = provider()
...
def _my_library_impl(ctx):
    ...
    my_info = MyInfo(x = 2, y = 3)
    # my_info.x == 2
    # my_info.y == 3
    ...</pre><p>See <a href='https://bazel.build/extending/rules#providers'>Rules (Providers)</a> for a comprehensive guide on how to use providers.<p>Returns a <a href='../builtins/Provider.html'><code>Provider</code></a> callable value if <code>init</code> is not specified.<p>If <code>init</code> is specified, returns a tuple of 2 elements: a <a href='../builtins/Provider.html'><code>Provider</code></a> callable value and a <em>raw constructor</em> callable value. See <a href='https://bazel.build/extending/rules#custom_initialization_of_providers'> Rules (Custom initialization of custom providers)</a> and the discussion of the <code>init</code> parameter below for details.(Ök
rule˜g
à
implementation;<a class="anchor" href="../core/function.html">function</a>∂the Starlark function implementing this rule, must have exactly one parameter: <a href="../builtins/ctx.html">ctx</a>. The function is called during the analysis phase for each instance of the rule. It can access the attributes provided by the user. It must create actions to generate all the declared outputs.(
ò
test3<a class="anchor" href="../core/bool.html">bool</a>—Whether this rule is a test rule, that is, whether it may be the subject of a <code>bazel test</code> command. All test rules are automatically considered <a href='#rule.executable'>executable</a>; it is unnecessary (and discouraged) to explicitly set <code>executable = True</code> for a test rule. The value defaults to <code>False</code>. See the <a href='https://bazel.build/extending/rules#executable_rules_and_test_rules'> Rules page</a> for more information."unbound
∫
attrs3<a class="anchor" href="../core/dict.html">dict</a>˜A dictionary to declare all the attributes of the rule. It maps from an attribute name to an attribute object (see
<a href="../toplevel/attr.html"><code>attr</code></a> module). Attributes starting with <code>_</code> are private, and can be used to add an implicit dependency on a label. The attribute <code>name</code> is implicitly added and must not be specified. Attributes <code>visibility</code>, <code>deprecation</code>, <code>tags</code>, <code>testonly</code>, and <code>features</code> are implicitly added and cannot be overridden. Most rules need only a handful of attributes. To limit memory usage, there is a cap on the number of attributes that may be declared.
<p>Declared attributes will convert <code>None</code> to the default value.</p>
"{}
ˇ
outputsâ<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>; or <a class="anchor" href="../core/function.html">function</a>·<b>Deprecated</b>. This parameter is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>--incompatible_no_rule_outputs_param</code>. Use this flag to verify your code is compatible with its imminent removal. <br>This parameter has been deprecated. Migrate rules to use <code>OutputGroupInfo</code> or <code>attr.output</code> instead. <p>A schema for defining predeclared outputs. Unlike <a href='../toplevel/attr.html#output'><code>output</code></a> and <a href='../toplevel/attr.html#output_list'><code>output_list</code></a> attributes, the user does not specify the labels for these files. See the <a href='https://bazel.build/extending/rules#files'>Rules page</a> for more on predeclared outputs.<p>The value of this argument is either a dictionary or a callback function that produces a dictionary. The callback works similar to computed dependency attributes: The function's parameter names are matched against the rule's attributes, so for example if you pass <code>outputs = _my_func</code> with the definition <code>def _my_func(srcs, deps): ...</code>, the function has access to the attributes <code>srcs</code> and <code>deps</code>. Whether the dictionary is specified directly or via a function, it is interpreted as follows.<p>Each entry in the dictionary creates a predeclared output where the key is an identifier and the value is a string template that determines the output's label. In the rule's implementation function, the identifier becomes the field name used to access the output's <a href='../builtins/File.html'><code>File</code></a> in <a href='../builtins/ctx.html#outputs'><code>ctx.outputs</code></a>. The output's label has the same package as the rule, and the part after the package is produced by substituting each placeholder of the form <code>"%{ATTR}"</code> with a string formed from the value of the attribute <code>ATTR</code>:<ul><li>String-typed attributes are substituted verbatim.<li>Label-typed attributes become the part of the label after the package, minus the file extension. For example, the label <code>"//pkg:a/b.c"</code> becomes <code>"a/b"</code>.<li>Output-typed attributes become the part of the label after the package, including the file extension (for the above example, <code>"a/b.c"</code>).<li>All list-typed attributes (for example, <code>attr.label_list</code>) used in placeholders are required to have <i>exactly one element</i>. Their conversion is the same as their non-list version (<code>attr.label</code>).<li>Other attribute types may not appear in placeholders.<li>The special non-attribute placeholders <code>%{dirname}</code> and <code>%{basename}</code> expand to those parts of the rule's label, excluding its package. For example, in <code>"//pkg:a/b.c"</code>, the dirname is <code>a</code> and the basename is <code>b.c</code>.</ul><p>In practice, the most common substitution placeholder is <code>"%{name}"</code>. For example, for a target named "foo", the outputs dict <code>{"bin": "%{name}.exe"}</code> predeclares an output named <code>foo.exe</code> that is accessible in the implementation function as <code>ctx.outputs.bin</code>."None
„

executable3<a class="anchor" href="../core/bool.html">bool</a>ñWhether this rule is considered executable, that is, whether it may be the subject of a <code>bazel run</code> command. It defaults to <code>False</code>. See the <a href='https://bazel.build/extending/rules#executable_rules_and_test_rules'> Rules page</a> for more information."unbound
´
output_to_genfiles3<a class="anchor" href="../core/bool.html">bool</a>ÿIf true, the files will be generated in the genfiles directory instead of the bin directory. Unless you need it for compatibility with existing rules (e.g. when generating header files for C++), do not set this flag."False
ﬁ
	fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sXList of names of configuration fragments that the rule requires in target configuration."[]
·
host_fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sVList of names of configuration fragments that the rule requires in host configuration."[]
ˆ
_skylark_testable3<a class="anchor" href="../core/bool.html">bool</a>§<i>(Experimental)</i><br/><br/>If true, this rule will expose its actions for inspection by rules that depend on it via an <code>Actions</code> provider. The provider is also available to the rule itself by calling <a href="../builtins/ctx.html#created_actions">ctx.created_actions()</a>.<br/><br/>This should only be used for testing the analysis-time behavior of Starlark rules. This flag may be removed in the future."False
›

toolchains7<a class="anchor" href="../core/list.html">sequence</a>ëIf set, the set of toolchains this rule requires. The list can contain String, Label, or StarlarkToolchainTypeApi objects, in any combination. Toolchains will be found by checking the current platform, and provided to the rule implementation via <code>ctx.toolchain</code>."[]
Æ
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>RA description of the rule that can be extracted by documentation generating tools."None
·
provides7<a class="anchor" href="../core/list.html">sequence</a>óA list of providers that the implementation function must return.<p>It is an error if the implementation function omits any of the types of providers listed here from its return value. However, the implementation function may return additional providers not listed here.<p>Each element of the list is an <code>*Info</code> object returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a>. When a target of the rule is used as a dependency for a target that declares a required provider, it is not necessary to specify that provider here. It is enough that the implementation function returns it. However, it is considered best practice to specify it, even though this is not required. The <a href='../globals/bzl.html#aspect.required_providers'><code>required_providers</code></a> field of an <a href='../globals/bzl.html#aspect'>aspect</a> does, however, require that providers are specified here."[]
Ï
dependency_resolution_rule3<a class="anchor" href="../core/bool.html">bool</a>ëIf set, the rule can be a dependency through attributes also marked as available in materializers. Every attribute of rules with this flag set must be marked as  available in materializers also. This is so that rules so marked cannot depend on rules that are not so marked."False
Ì
exec_compatible_withs<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s\A list of constraints on the execution platform that apply to all targets of this rule type."[]
Î
analysis_test3<a class="anchor" href="../core/bool.html">bool</a>ùIf true, then this rule is treated as an analysis test. <p>Note: Analysis test rules are primarily defined using infrastructure provided in core Starlark libraries. See <a href="https://bazel.build/rules/testing#testing-rules">Testing</a> for guidance. <p>If a rule is defined as an analysis test rule, it becomes allowed to use configuration transitions defined using <a href="#analysis_test_transition">analysis_test_transition</a> on its attributes, but opts into some restrictions: <ul><li>Targets of this rule are limited in the number of transitive dependencies they may have. <li>The rule is considered a test rule (as if <code>test=True</code> were set). This supersedes the value of <code>test</code></li> <li>The rule implementation function may not register actions. Instead, it must register a pass/fail result via providing <a href='../providers/AnalysisTestResultInfo.html'>AnalysisTestResultInfo</a>.</li></ul>"False
‡
build_setting]<a class="anchor" href="../builtins/BuildSetting.html">BuildSetting</a>; or <code>None</code>ÈIf set, describes what kind of <a href='/rules/config#user-defined-build-settings'><code>build setting</code></a> this rule is. See the <a href='../toplevel/config.html'><code>config</code></a> module. If this is set, a mandatory attribute named "build_setting_default" is automatically added to this rule, with a type corresponding to the value passed in here."None
y
cfglIf set, points to the configuration transition the rule will apply to its own configuration before analysis."None
É
exec_groupsI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>¢Dict of execution group name (string) to <a href='../globals/bzl.html#exec_group'><code>exec_group</code>s</a>. If set, allows rules to run actions on multiple execution platforms within a single target. See <a href='/reference/exec-groups'>execution groups documentation</a> for more info."None
£

initializerç
Experimental: the Stalark function initializing the attributes of the rule. <p>The function is called at load time for each instance of the rule. It's called with <code>name</code> and the values of public attributes defined by the rule (not with generic attributes, for example <code>tags</code>). <p>It has to return a dictionary from the attribute names to the desired values. The attributes that are not returned are unaffected. Returning <code>None</code> as value results in using the default value specified in the attribute definition. <p>Initializers are evaluated before the default values specified in an attribute definition. Consequently, if a parameter in the initializer's signature contains a default values, it overwrites the default from the attribute definition (except if returning <code>None</code>). <p>Similarly, if a parameter in the initializer's signature doesn't have a default, the parameter will become mandatory. It's a good practice to omit default/mandatory settings on an attribute definition in such cases. <p>It's a good practice to use <code>**kwargs</code> for attributes that are not handled.<p>In case of extended rules, all initializers are called proceeding from child to ancestors. Each initializer is passed only the public attributes it knows about."None
Ñ
parentÛExperimental: the Stalark rule that is extended. When set the public attributes are merged as well as advertised providers. The rule matches <code>executable</code> and <code>test</code> from the parent. Values of <code>fragments</code>, <code>toolchains</code>, <code>exec_compatible_with</code>, and <code>exec_groups</code> are merged. Legacy or deprecated parameters may not be set. Incoming configuration transition <code>cfg</code> of parent is applied after thisrule's incoming configuration."None
°

extendable√<a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>∆Experimental: A label of an allowlist defining which rules can extending this rule. It can be set also to True/False to always allow/disallow extending. Bazel defaults to always allowing extensions."None
º
subrulesy<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Subrule.html">Subrule</a>s1Experimental: List of subrules used by this rule."[]callable"ÄCreates a new rule, which can be called from a BUILD file or a macro to create targets.<p>Rules must be assigned to global variables in a .bzl file; the name of the global variable is the rule's name.<p>Test rules are required to have a name ending in <code>_test</code>, while all other rules must not have this suffix. (This restriction applies only to rules, not to their targets.)(ü
subruleë
Ä
implementation;<a class="anchor" href="../core/function.html">function</a>/The Starlark function implementing this subrule(
π
attrs3<a class="anchor" href="../core/dict.html">dict</a>ˆA dictionary to declare all the (private) attributes of the subrule. <p/>Subrules may only have private attributes that are label-typed (i.e. label or label-list). The resolved values corresponding to these labels are automatically passed by Bazel to the subrule's implementation function as named arguments (thus the implementation function is required to accept named parameters matching the attribute names). The types of these values will be: <ul><li><code>FilesToRunProvider</code> for label attributes with <code>executable=True</code></li><li><code>File</code> for label attributes with <code>allow_single_file=True</code></li><li><code>Target</code> for all other label attributes</li><li><code>[Target]</code> for all label-list attributes</li></ul>"{}
¶

toolchains7<a class="anchor" href="../core/list.html">sequence</a>⁄If set, the set of toolchains this subrule requires. The list can contain String, Label, or StarlarkToolchainTypeApi objects, in any combination. Toolchains will be found by checking the current platform, and provided to the subrule implementation via <code>ctx.toolchains</code>. Note that AEGs need to be enabled on the consuming rule(s) if this parameter is set. In case you haven't migrated to AEGs yet, see https://bazel.build/extending/auto-exec-groups#migration-aegs."[]
·
	fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s[List of names of configuration fragments that the subrule requires in target configuration."[]
π
subrulesy<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Subrule.html">Subrule</a>s.List of other subrules needed by this subrule."[]Subrule"~Constructs a new instance of a subrule. The result of this function must be stored in a global variable before it can be used.(Ï
attrattr"€This is a top-level module for defining the attribute schemas of a rule or aspect. Each function returns an object representing the schema of a single attribute. These objects are used as the values of the <code>attrs</code> dictionary argument of <a href="../globals/bzl.html#rule"><code>rule()</code></a>, <a href="../globals/bzl.html#aspect"><code>aspect()</code></a>, <a href="../globals/bzl.html#repository_rule"><code>repository_rule()</code></a> and <a href="../globals/bzl.html#tag_class"><code>tag_class()</code></a>. <p>See the Rules page for more on <a href="https://bazel.build/extending/rules#attributes">defining</a>
and <a href="https://bazel.build/extending/rules#implementation_function">using</a> attributes.</p>
(ã
struct

**kwargs8struct"ÊCreates an immutable struct using the keyword arguments as attributes. It is used to group multiple values together. Example:<br><pre class="language-python">s = struct(x = 2, y = 3)
return s.x + getattr(s, "y")  # returns 5</pre>(π
OutputGroupInfo

**kwargs8OutputGroupInfo"ÇInstantiate this provider with <br><pre class=language-python>OutputGroupInfo(group1 = &lt;files&gt;, group2 = &lt;files&gt;...)</pre>See <a href="https://bazel.build/extending/rules#requesting_output_files">Requesting output files </a> for more information.([
ActionsActions"E<b>Deprecated and subject to imminent removal. Please do not use.</b>(Ø
DefaultInfos

files"None

runfiles"None

data_runfiles"None

default_runfiles"None


executable"NoneDefaultInfo")The <code>DefaultInfo</code> constructor.(\
RunEnvironmentInfoD

environment"{}

inherited_environment"[]RunEnvironmentInfo(n
MaterializedDepsInfo 

deps(MaterializedDepsInfo"2The <code>MaterializedDepsInfo</code> constructor.((i
proto_common_do_not_useproto_common_do_not_use"3Private utilities for protocol buffers. Do not use.(8
CcInfoCcInfo"$The type of the Starlark None value.(L
DebugPackageInfoDebugPackageInfo"$The type of the Starlark None value.(R
CcSharedLibraryInfoCcSharedLibraryInfo"$The type of the Starlark None value.(Z
CcSharedLibraryHintInfoCcSharedLibraryHintInfo"$The type of the Starlark None value.(B
java_commonjava_common"$The type of the Starlark None value.(‡
android_commonandroid_common"ªDo not use this module. It is intended for migration purposes only. If you depend on it, you will be broken when it is removed.Common utilities and functionality related to Android rules.(B
py_internalpy_internal"$The type of the Starlark None value.(ê
PackageSpecificationInfoPackageSpecificationInfo"◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.(g
config_commonconfig_common"EFunctions for Starlark to interact with Blaze's configurability APIs.(‰
configconfig"œThis is a top-level module for creating configuration transitions and build setting descriptors which describe what kind of build setting (if any) a rule is. <p>ex: the following rule is marked as a build setting by setting the <code>build_setting</code> parameter of the <code>rule()</code> function. Specifically it is a build setting of type <code>int</code> and is a <code>flag</code> which means this build setting is callable on the command line.<br><pre class=language-python>  my_rule = rule(
    implementation = _impl,
    build_setting = config.int(flag = True),
    ...
  )</pre>(˚
analysis_test_transition©
ö
settings3<a class="anchor" href="../core/dict.html">dict</a>÷A dictionary containing information about configuration settings which should be set by this configuration transition. Keys are build setting labels and values are their new post-transition values. All other settings are unchanged. Use this to declare specific configuration settings that an analysis test requires to be set in order to pass.(
transition"∞<p> Creates a configuration transition to be applied on an analysis-test rule's dependencies. This transition may only be applied on attributes of rules with <code>analysis_test = True</code>. Such rules are restricted in capabilities (for example, the size of their dependency tree is limited), so transitions created using this function are limited in potential scope as compared to transitions created using <a href="../builtins/transition.html"><code>transition()</code></a>. <p>This function is primarily designed to facilitate the <a href="https://bazel.build/rules/testing">Analysis Test Framework</a> core library. See its documentation (or its implementation) for best practices.(Æ
exec_transitionÆ

implementationcallable(

inputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s(
Ä
outputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s(
transition"ÁA specialized version of <a href="../builtins/transition.html"><code>transition()</code></a> used to define the exec transition. See its documentation (or its implementation) for best practices. Only usable from the Bazel builtins.(¢

transition-

implementation(


inputs(

outputs("‚A transition that reads a set of input build settings and writes a set of output build settings.<p>Example:</p><p><pre class="language-python">
def _transition_impl(settings, attr):
    # This transition just reads the current CPU value as a demonstration.
    # A real transition could incorporate this into its followup logic.
    current_cpu = settings["//command_line_option:cpu"]
    return {"//command_line_option:compilation_mode": "dbg"}

build_in_debug_mode = transition(
    implementation = _transition_impl,
    inputs = ["//command_line_option:cpu"],
    outputs = ["//command_line_option:compilation_mode"],
)</pre></p><p>For more details see <a href="https://bazel.build/rules/config#user-defined-transitions">here</a>.</p>(`
platform_commonplatform_common":Functions for Starlark to interact with the platform APIs.(>
	cc_common	cc_common"$The type of the Starlark None value.(V
CcToolchainConfigInfoCcToolchainConfigInfo"$The type of the Starlark None value.(D
apple_commonapple_common"$The type of the Starlark None value.(S
testingtesting"=Helper methods for Starlark to access testing infrastructure.(a
coverage_commoncoverage_common";Helper functions to access coverage-related infrastructure.(ä
InstrumentedFilesInfoInstrumentedFilesInfo"◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.(,
AnalysisFailureInfoAnalysisFailureInfo(Ñ
AnalysisTestResultInfo2

success(

message(AnalysisTestResultInfo"4The <code>AnalysisTestResultInfo</code> constructor.(ó
	cc_binaryˆ

name(

deps

srcs

data

additional_linker_inputs

args

aspect_hints

compatible_with

	conlyopts

copts
	
cxxopts
	
defines

deprecation


distribs

dynamic_deps

env

exec_compatible_with

exec_group_compatible_with

exec_properties


features


hdrs_check


includes


licenses

link_extra_lib


linkopts


linkshared


linkstatic

local_defines

malloc

module_interfaces
	
nocopts

output_licenses

package_metadata

reexport_deps

restricted_to

stamp

tags

target_compatible_with


testonly


toolchains


visibility

win_def_file

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies

$bzl_load_label"é<p>It produces an executable binary.</p>

<br/>The <code>name</code> of the target should be the same as the name of the
source file that is the main entry point of the application (minus the extension).
For example, if your entry point is in <code>main.cc</code>, then your name should
be <code>main</code>.

<h4>Implicit output targets</h4>
<ul>
<li><code><var>name</var>.stripped</code> (only built if explicitly requested): A stripped
  version of the binary. <code>strip -g</code> is run on the binary to remove debug
  symbols.  Additional strip options can be provided on the command line using
  <code>--stripopt=-foo</code>.</li>
<li><code><var>name</var>.dwp</code> (only built if explicitly requested): If
  <a href="https://gcc.gnu.org/wiki/DebugFission">Fission</a> is enabled: a debug
  information package file suitable for debugging remotely deployed binaries. Else: an
  empty file.</li>
</ul>(€
	cc_import˝

name(

deps

data

hdrs


alwayslink

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


includes

interface_library


linkopts
	
objects

package_metadata

pic_objects

pic_static_library

restricted_to

shared_library

static_library

strip_include_prefix

system_provided

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"À<p>
<code>cc_import</code> rules allows users to import precompiled C/C++ libraries.
</p>

<p>
The following are the typical use cases: <br/>

1. Linking a static library
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  static_library = "libmylib.a",
  # If alwayslink is turned on,
  # libmylib.a will be forcely linked into any binary that depends on it.
  # alwayslink = 1,
)
</code></pre>

2. Linking a shared library (Unix)
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  shared_library = "libmylib.so",
)
</code></pre>

3. Linking a shared library with interface library

<p>On Unix:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  # libmylib.ifso is an interface library for libmylib.so which will be passed to linker
  interface_library = "libmylib.ifso",
  # libmylib.so will be available for runtime
  shared_library = "libmylib.so",
)
</code></pre>

<p>On Windows:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  # mylib.lib is an import library for mylib.dll which will be passed to linker
  interface_library = "mylib.lib",
  # mylib.dll will be available for runtime
  shared_library = "mylib.dll",
)
</code></pre>

4. Linking a shared library with <code>system_provided=True</code>

<p>On Unix:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  interface_library = "libmylib.ifso", # Or we can also use libmylib.so as its own interface library
  # libmylib.so is provided by system environment, for example it can be found in LD_LIBRARY_PATH.
  # This indicates that Bazel is not responsible for making libmylib.so available.
  system_provided = 1,
)
</code></pre>

<p>On Windows:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  # mylib.lib is an import library for mylib.dll which will be passed to linker
  interface_library = "mylib.lib",
  # mylib.dll is provided by system environment, for example it can be found in PATH.
  # This indicates that Bazel is not responsible for making mylib.dll available.
  system_provided = 1,
)
</code></pre>

5. Linking to static or shared library

<p>On Unix:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  static_library = "libmylib.a",
  shared_library = "libmylib.so",
)
</code></pre>

<p>On Windows:
<pre><code class="lang-starlark">
cc_import(
  name = "mylib",
  hdrs = ["mylib.h"],
  static_library = "libmylib.lib", # A normal static library
  interface_library = "mylib.lib", # An import library for mylib.dll
  shared_library = "mylib.dll",
)
</code></pre>

<p>The remaining is the same on Unix and Windows:
<pre><code class="lang-starlark">
# first will link to libmylib.a (or libmylib.lib)
cc_binary(
  name = "first",
  srcs = ["first.cc"],
  deps = [":mylib"],
  linkstatic = 1, # default value
)

# second will link to libmylib.so (or libmylib.lib)
cc_binary(
  name = "second",
  srcs = ["second.cc"],
  deps = [":mylib"],
  linkstatic = 0,
)
</code></pre>

<p>
<code>cc_import</code> supports an include attribute. For example:
<pre><code class="lang-starlark">
cc_import(
  name = "curl_lib",
  hdrs = glob(["vendor/curl/include/curl/*.h"]),
  includes = ["vendor/curl/include"],
  shared_library = "vendor/curl/lib/.libs/libcurl.dylib",
)
</code></pre>
</p>(ı;

cc_libraryá

name(

deps

srcs

data

hdrs

additional_compiler_inputs

additional_linker_inputs


alwayslink

aspect_hints

compatible_with

	conlyopts

copts
	
cxxopts
	
defines

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


hdrs_check

implementation_deps

include_prefix


includes


licenses


linkopts

	linkstamp


linkstatic

local_defines

module_interfaces

package_metadata

restricted_to

strip_include_prefix

tags

target_compatible_with


testonly

textual_hdrs


toolchains


visibility

win_def_file

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"⁄5<p>Use <code>cc_library()</code> for C++-compiled libraries.
  The result is  either a <code>.so</code>, <code>.lo</code>,
  or <code>.a</code>, depending on what is needed.
</p>

<p>
  If you build something with static linking that depends on
  a <code>cc_library</code>, the output of a depended-on library rule
  is the <code>.a</code> file. If you specify
   <code>alwayslink=True</code>, you get the <code>.lo</code> file.
</p>

<p>
  The actual output file name is <code>lib<i>foo</i>.so</code> for
  the shared library, where <i>foo</i> is the name of the rule.  The
  other kinds of libraries end with <code>.lo</code> and <code>.a</code>,
  respectively.  If you need a specific shared library name, for
  example, to define a Python module, use a genrule to copy the library
  to the desired name.
</p>

<h4 id="hdrs">Header inclusion checking</h4>

<p>
  All header files that are used in the build must be declared in
  the <code>hdrs</code> or <code>srcs</code> of <code>cc_*</code> rules.
  This is enforced.
</p>

<p>
  For <code>cc_library</code> rules, headers in <code>hdrs</code> comprise the
  public interface of the library and can be directly included both
  from the files in <code>hdrs</code> and <code>srcs</code> of the library
  itself as well as from files in <code>hdrs</code> and <code>srcs</code>
  of <code>cc_*</code> rules that list the library in their <code>deps</code>.
  Headers in <code>srcs</code> must only be directly included from the files
  in <code>hdrs</code> and <code>srcs</code> of the library itself. When
  deciding whether to put a header into <code>hdrs</code> or <code>srcs</code>,
  you should ask whether you want consumers of this library to be able to
  directly include it. This is roughly the same decision as
  between <code>public</code> and <code>private</code> visibility in programming languages.
</p>

<p>
  <code>cc_binary</code> and <code>cc_test</code> rules do not have an exported
  interface, so they also do not have a <code>hdrs</code> attribute. All headers
  that belong to the binary or test directly should be listed in
  the <code>srcs</code>.
</p>

<p>
  To illustrate these rules, look at the following example.
</p>

<pre><code class="lang-starlark">
cc_binary(
    name = "foo",
    srcs = [
        "foo.cc",
        "foo.h",
    ],
    deps = [":bar"],
)

cc_library(
    name = "bar",
    srcs = [
        "bar.cc",
        "bar-impl.h",
    ],
    hdrs = ["bar.h"],
    deps = [":baz"],
)

cc_library(
    name = "baz",
    srcs = [
        "baz.cc",
        "baz-impl.h",
    ],
    hdrs = ["baz.h"],
)
</code></pre>

<p>
  The allowed direct inclusions in this example are listed in the table below.
  For example <code>foo.cc</code> is allowed to directly
  include <code>foo.h</code> and <code>bar.h</code>, but not <code>baz.h</code>.
</p>

<table class="table table-striped table-bordered table-condensed">
  <thead>
    <tr><th>Including file</th><th>Allowed inclusions</th></tr>
  </thead>
  <tbody>
    <tr><td>foo.h</td><td>bar.h</td></tr>
    <tr><td>foo.cc</td><td>foo.h bar.h</td></tr>
    <tr><td>bar.h</td><td>bar-impl.h baz.h</td></tr>
    <tr><td>bar-impl.h</td><td>bar.h baz.h</td></tr>
    <tr><td>bar.cc</td><td>bar.h bar-impl.h baz.h</td></tr>
    <tr><td>baz.h</td><td>baz-impl.h</td></tr>
    <tr><td>baz-impl.h</td><td>baz.h</td></tr>
    <tr><td>baz.cc</td><td>baz.h baz-impl.h</td></tr>
  </tbody>
</table>

<p>
  The inclusion checking rules only apply to <em>direct</em>
  inclusions. In the example above <code>foo.cc</code> is allowed to
  include <code>bar.h</code>, which may include <code>baz.h</code>, which in
  turn is allowed to include <code>baz-impl.h</code>. Technically, the
  compilation of a <code>.cc</code> file may transitively include any header
  file in the <code>hdrs</code> or <code>srcs</code> in
  any <code>cc_library</code> in the transitive <code>deps</code> closure. In
  this case the compiler may read <code>baz.h</code> and <code>baz-impl.h</code>
  when compiling <code>foo.cc</code>, but <code>foo.cc</code> must not
  contain <code>#include "baz.h"</code>. For that to be
  allowed, <code>baz</code> must be added to the <code>deps</code>
  of <code>foo</code>.
</p>

<p>
  Bazel depends on toolchain support to enforce the inclusion checking rules.
  The <code>layering_check</code> feature has to be supported by the toolchain
  and requested explicitly, for example via the
  <code>--features=layering_check</code> command-line flag or the
  <code>features</code> parameter of the
  <a href="#package"><code>package</code></a> function. The toolchains
  provided by Bazel only support this feature with clang on Unix and macOS.
</p>

<h4 id="cc_library_examples">Examples</h4>

<p id="alwayslink_lib_example">
   We use the <code>alwayslink</code> flag to force the linker to link in
   this code although the main binary code doesn't reference it.
</p>

<pre><code class="lang-starlark">
cc_library(
    name = "ast_inspector_lib",
    srcs = ["ast_inspector_lib.cc"],
    hdrs = ["ast_inspector_lib.h"],
    visibility = ["//visibility:public"],
    deps = ["//third_party/llvm/llvm/tools/clang:frontend"],
    # alwayslink as we want to be able to call things in this library at
    # debug time, even if they aren't used anywhere in the code.
    alwayslink = 1,
)
</code></pre>


<p>The following example comes from
   <code>third_party/python2_4_3/BUILD</code>.
   Some of the code uses the <code>dl</code> library (to load
   another, dynamic library), so this
   rule specifies the <code>-ldl</code> link option to link the
   <code>dl</code> library.
</p>

<pre><code class="lang-starlark">
cc_library(
    name = "python2_4_3",
    linkopts = [
        "-ldl",
        "-lutil",
    ],
    deps = ["//third_party/expat"],
)
</code></pre>

<p>The following example comes from <code>third_party/kde/BUILD</code>.
   We keep pre-built <code>.so</code> files in the depot.
   The header files live in a subdirectory named <code>include</code>.
</p>

<pre><code class="lang-starlark">
cc_library(
    name = "kde",
    srcs = [
        "lib/libDCOP.so",
        "lib/libkdesu.so",
        "lib/libkhtml.so",
        "lib/libkparts.so",
        <var>...more .so files...</var>,
    ],
    includes = ["include"],
    deps = ["//third_party/X11"],
)
</code></pre>

<p>The following example comes from <code>third_party/gles/BUILD</code>.
   Third-party code often needs some <code>defines</code> and
   <code>linkopts</code>.
</p>

<pre><code class="lang-starlark">
cc_library(
    name = "gles",
    srcs = [
        "GLES/egl.h",
        "GLES/gl.h",
        "ddx.c",
        "egl.c",
    ],
    defines = [
        "USE_FLOAT",
        "__GL_FLOAT",
        "__GL_COMMON",
    ],
    linkopts = ["-ldl"],  # uses dlopen(), dl library
    deps = [
        "es",
        "//third_party/X11",
    ],
)
</code></pre>(Ô,
cc_shared_library˛

name(

deps

additional_linker_inputs

aspect_hints

compatible_with

deprecation

dynamic_deps

exec_compatible_with

exec_group_compatible_with

exec_properties
=
;experimental_disable_topo_sort_do_not_use_remove_before_7_0

exports_filter


features

package_metadata

restricted_to

roots

shared_lib_name

static_deps

tags

target_compatible_with


testonly


toolchains

user_link_flags


visibility

win_def_file

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"÷'<p>It produces a shared library.</p>

<h4 id="cc_shard_library_examples">Example</h4>

<pre class="code">
cc_shared_library(
    name = "foo_shared",
    deps = [
        ":foo",
    ],
    dynamic_deps = [
        ":bar_shared",
    ],
    additional_linker_inputs = [
        ":foo.lds",
    ],
    user_link_flags = [
        "-Wl,--version-script=$(location :foo.lds)",
    ],
)
cc_library(
    name = "foo",
    srcs = ["foo.cc"],
    hdrs = ["foo.h"],
    deps = [
        ":bar",
        ":baz",
    ],
)
cc_shared_library(
    name = "bar_shared",
    shared_lib_name = "bar.so",
    deps = [":bar"],
)
cc_library(
    name = "bar",
    srcs = ["bar.cc"],
    hdrs = ["bar.h"],
)
cc_library(
    name = "baz",
    srcs = ["baz.cc"],
    hdrs = ["baz.h"],
)
</pre>

<p>In the example <code>foo_shared</code> statically links <code>foo</code>
and <code>baz</code>, the latter being a transitive dependency. It doesn't
link <code>bar</code> because it is already provided dynamically by the
<code>dynamic_dep</code> <code>bar_shared</code>.</p>

<p><code>foo_shared</code> uses a linker script *.lds file to control which
symbols should be exported. The <code>cc_shared_library</code> rule logic does
not control which symbols get exported, it only uses what is assumed to be
exported to give errors during analysis phase if two shared libraries export the
same targets.</p>

<p>Every direct dependency of <code>cc_shared_library</code> is assumed to be
exported. Therefore, Bazel assumes during analysis that <code>foo</code> is being
exported by <code>foo_shared</code>. <code>baz</code> is not assumed to be exported
by <code>foo_shared</code>. Every target matched by the <code>exports_filter</code>
is also assumed to be exported.</p>

<p>Every single <code>cc_library</code> in the example should appear at most in one
<code>cc_shared_library</code>. If we wanted to link <code>baz</code> also into
<code>bar_shared</code> we would need to add
<code>tags = ["LINKABLE_MORE_THAN_ONCE"]</code> to <code>baz</code>.</p>

<p>Due to the <code>shared_lib_name</code> attribute, the file produced by
<code>bar_shared</code> will have the name <code>bar.so</code> as opposed
to the name <code>libbar.so</code> that it would have by default on Linux.</p>

<h4 id="cc_shard_library_examples">Errors</h4>
<h5><code>Two shared libraries in dependencies export the same symbols.</code></h5>

<p>This will happen whenever you are creating a target with two different
<code>cc_shared_library</code> dependencies that export the same target. To fix this
you need to stop the libraries from being exported in one of the
<code>cc_shared_library</code> dependencies.</p>

<h5><code>Two shared libraries in dependencies link the same library statically</code></h5>

<p>This will happen whenever you are creating a new <code>cc_shared_library</code> with two
different <code>cc_shared_library</code> dependencies that link the same target statically.
Similar to the error with exports.</p>

<p>One way to fix this is to stop linking the library into one of the
<code>cc_shared_library</code> dependencies. At the same time, the one that still links it
needs to export the library so that the one not linking it keeps visibility to
the symbols. Another way is to pull out a third library that exports the target.
A third way is to tag the culprit <code>cc_library</code> with <code>LINKABLE_MORE_THAN_ONCE</code>
but this fix should be rare and you should absolutely make sure that the
<code>cc_library</code> is indeed safe to link more than once.</p>

<h5><code>'//foo:foo' is already linked statically in '//bar:bar' but not exported`</code></h5>

<p>This means that a library in the transitive closure of your <code>deps</code> is reachable
without going through one of the <code>cc_shared_library</code> dependencies but is already
linked into a different <code>cc_shared_library</code> in <code>dynamic_deps</code> and is not
exported.</p>

<p>The solution is to export it from the <code>cc_shared_library</code> dependency or pull out
a third <code>cc_shared_library</code> that exports it.</p>

<h5><code>Do not place libraries which only contain a precompiled dynamic library in deps.
</code></h5>

<p>If you have a precompiled dynamic library, this doesn't need to and cannot be
linked statically into the current <code>cc_shared_library</code> target that you are
currently creating. Therefore, it doesn't belong in <code>deps</code> of the
<code>cc_shared_library</code>. If this precompiled dynamic library is a dependency of one
of your <code>cc_libraries</code>, then the <code>cc_library</code> needs to depend on it
directly.</p>

<h5><code>Trying to export a library already exported by a different shared library</code></h5>

<p>You will see this error if on the current rule you are claiming to export a
target that is already being exported by one of your dynamic dependencies.</p>

<p>To fix this, remove the target from <code>deps</code> and just rely on it from the dynamic
dependency or make sure that the <code>exports_filter</code> doesn't catch this target.</p>(¥
cc_static_library≥

name(

deps

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"ÊProduces a static library from a list of targets and their transitive dependencies.

<p>The resulting static library contains the object files of the targets listed in
<code>deps</code> as well as their transitive dependencies, with preference given to
<code>PIC</code> objects.</p>

<h4 id="cc_static_library_output_groups">Output groups</h4>

<h5><code>linkdeps</code></h5>
<p>A text file containing the labels of those transitive dependencies of targets listed in
<code>deps</code> that did not contribute any object files to the static library, but do
provide at least one static, dynamic or interface library. The resulting static library
may require these libraries to be available at link time.</p>

<h5><code>linkopts</code></h5>
<p>A text file containing the user-provided <code>linkopts</code> of all transitive
dependencies of targets listed in <code>deps</code>.

<h4 id="cc_static_library_symbol_check">Duplicate symbols</h4>
<p>By default, the <code>cc_static_library</code> rule checks that the resulting static
library does not contain any duplicate symbols. If it does, the build fails with an error
message that lists the duplicate symbols and the object files containing them.</p>

<p>This check can be disabled per target or per package by setting
<code>features = ["-symbol_check"]</code> or globally via
<code>--features=-symbol_check</code>.</p>

<h5 id="cc_static_library_symbol_check_toolchain">Toolchain support for <code>symbol_check</code></h5>
<p>The auto-configured C++ toolchains shipped with Bazel support the
<code>symbol_check</code> feature on all platforms. Custom toolchains can add support for
it in one of two ways:</p>
<ul>
  <li>Implementing the <code>ACTION_NAMES.validate_static_library</code> action and
  enabling it with the <code>symbol_check</code> feature. The tool set in the action is
  invoked with two arguments, the static library to check for duplicate symbols and the
  path of a file that must be created if the check passes.</li>
  <li>Having the <code>symbol_check</code> feature add archiver flags that cause the
  action creating the static library to fail on duplicate symbols.</li>
</ul>(¡
cc_test¶

name(

deps

srcs

data

additional_linker_inputs

args

aspect_hints

compatible_with

	conlyopts

copts
	
cxxopts
	
defines

deprecation


distribs

dynamic_deps

env

env_inherit

exec_compatible_with

exec_group_compatible_with

exec_properties


features

flaky


hdrs_check


includes


licenses

link_extra_lib


linkopts


linkshared


linkstatic

local

local_defines

malloc

module_interfaces
	
nocopts

package_metadata

reexport_deps

restricted_to

shard_count

size

stamp

tags

target_compatible_with


testonly
	
timeout


toolchains


visibility

win_def_file

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies

$bzl_load_label"ä
<p>
A <code>cc_test()</code> rule compiles a test.  Here, a test
is a binary wrapper around some testing code.
</p>

<p><i>By default, C++ tests are dynamically linked.</i><br/>
    To statically link a unit test, specify
    <a href="#cc_binary.linkstatic"><code>linkstatic=True</code></a>.
    It would probably be good to comment why your test needs
    <code>linkstatic</code>; this is probably not obvious.</p>

<h4>Implicit output targets</h4>
<ul>
<li><code><var>name</var>.stripped</code> (only built if explicitly requested): A stripped
  version of the binary. <code>strip -g</code> is run on the binary to remove debug
  symbols.  Additional strip options can be provided on the command line using
  <code>--stripopt=-foo</code>.</li>
<li><code><var>name</var>.dwp</code> (only built if explicitly requested): If
  <a href="https://gcc.gnu.org/wiki/DebugFission">Fission</a> is enabled: a debug
  information package file suitable for debugging remotely deployed binaries. Else: an
  empty file.</li>
</ul>

<p>
See the <a href="#cc_binary_args">cc_binary()</a> arguments, except that
the <code>stamp</code> argument is set to 0 by default for tests and
that <code>cc_test</code> has extra <a href="#common-attributes-tests">
attributes common to all test rules (*_test)</a>.</p>(Œ
cc_toolchainπ

name(

	all_files(


ar_files


as_files

aspect_hints

compatible_with

compiler_files(
!
compiler_files_without_includes

coverage_files

deprecation

	dwp_files(

dynamic_runtime_lib

exec_compatible_with

exec_group_compatible_with

exec_properties

exec_transition_for_inputs


features


libc_top


licenses

linker_files(


module_map

objcopy_files(

output_licenses

package_metadata

restricted_to

static_runtime_lib

strip_files(

supports_header_parsing

supports_param_files

tags

target_compatible_with


testonly

toolchain_config(

toolchain_identifier


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"ˇ<p>Represents a C++ toolchain.</p>

<p>
  This rule is responsible for:

  <ul>
    <li>
      Collecting all artifacts needed for C++ actions to run. This is done by
      attributes such as <code>all_files</code>, <code>compiler_files</code>,
      <code>linker_files</code>, or other attributes ending with <code>_files</code>). These are
      most commonly filegroups globbing all required files.
    </li>
    <li>
      Generating correct command lines for C++ actions. This is done using
      <code>CcToolchainConfigInfo</code> provider (details below).
    </li>
  </ul>
</p>
<p>
  Use <code>toolchain_config</code> attribute to configure the C++ toolchain.
  See also this
  <a href="https://bazel.build/docs/cc-toolchain-config-reference">
    page
  </a> for elaborate C++ toolchain configuration and toolchain selection documentation.
</p>
<p>
  Use <code>tags = ["manual"]</code> in order to prevent toolchains from being built and configured
  unnecessarily when invoking <code>bazel build //...</code>
</p>(∑
fdo_prefetch_hints∏

name(

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

profile(

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"„<p>Represents an FDO prefetch hints profile that is either in the workspace.
Examples:</p>

<pre><code class="lang-starlark">
fdo_prefetch_hints(
    name = "hints",
    profile = "//path/to/hints:profile.afdo",
)
</code></pre>(±
fdo_profile‹

name(

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

memprof_profile

package_metadata

profile(

proto_profile

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"¿<p>Represents an FDO profile that is in the workspace.
Example:</p>

<pre><code class="lang-starlark">
fdo_profile(
    name = "fdo",
    profile = "//path/to/fdo:profile.zip",
)
</code></pre>(¢
memprof_profile∏

name(

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

profile(

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"—<p>Represents a MEMPROF profile that is in the workspace.
Example:</p>

<pre><code class="lang-starlark">
memprof_profile(
    name = "memprof",
    profile = "//path/to/memprof:profile.afdo",
)

</code></pre>(„
propeller_optimizeÀ

name(

aspect_hints


cc_profile(

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


ld_profile(

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"¸<p>Represents a Propeller optimization profile in the workspace.
Example:</p>

<pre><code class="lang-starlark">
propeller_optimize(
    name = "layout",
    cc_profile = "//path:cc_profile.txt",
    ld_profile = "//path:ld_profile.txt"
)
</code></pre>(˛$
java_binaryí

name(

deps

srcs

data

	resources

add_exports

	add_opens

args

aspect_hints

bootclasspath

classpath_resources

compatible_with

create_executable


deploy_env

deploy_manifest_lines

deprecation

env

exec_compatible_with

exec_group_compatible_with

exec_properties


features

	javacopts

	jvm_flags


launcher


licenses


main_class

	neverlink

output_licenses

package_metadata
	
plugins

resource_strip_prefix

restricted_to

runtime_deps

stamp

tags

target_compatible_with


testonly


toolchains

use_launcher

use_testrunner


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"◊<p>
  Builds a Java archive ("jar file"), plus a wrapper shell script with the same name as the rule.
  The wrapper shell script uses a classpath that includes, among other things, a jar file for each
  library on which the binary depends. When running the wrapper shell script, any nonempty
  <code>JAVABIN</code> environment variable will take precedence over the version specified via
  Bazel's <code>--java_runtime_version</code> flag.
</p>
<p>
  The wrapper script accepts several unique flags. Refer to
  <code>java_stub_template.txt</code>
  for a list of configurable flags and environment variables accepted by the wrapper.
</p>

<h4 id="java_binary_implicit_outputs">Implicit output targets</h4>
<ul>
  <li><code><var>name</var>.jar</code>: A Java archive, containing the class files and other
    resources corresponding to the binary's direct dependencies.</li>
  <li><code><var>name</var>-src.jar</code>: An archive containing the sources ("source
    jar").</li>
  <li><code><var>name</var>_deploy.jar</code>: A Java archive suitable for deployment (only
    built if explicitly requested).
    <p>
      Building the <code>&lt;<var>name</var>&gt;_deploy.jar</code> target for your rule
      creates a self-contained jar file with a manifest that allows it to be run with the
      <code>java -jar</code> command or with the wrapper script's <code>--singlejar</code>
      option. Using the wrapper script is preferred to <code>java -jar</code> because it
      also passes the <a href="#java_binary-jvm_flags">JVM flags</a> and the options
      to load native libraries.
    </p>
    <p>
      The deploy jar contains all the classes that would be found by a classloader that
      searched the classpath from the binary's wrapper script from beginning to end. It also
      contains the native libraries needed for dependencies. These are automatically loaded
      into the JVM at runtime.
    </p>
    <p>If your target specifies a <a href="#java_binary.launcher">launcher</a>
      attribute, then instead of being a normal JAR file, the _deploy.jar will be a
      native binary. This will contain the launcher plus any native (C++) dependencies of
      your rule, all linked into a static binary. The actual jar file's bytes will be
      appended to that native binary, creating a single binary blob containing both the
      executable and the Java code. You can execute the resulting jar file directly
      like you would execute any native binary.</p>
  </li>
  <li><code><var>name</var>_deploy-src.jar</code>: An archive containing the sources
    collected from the transitive closure of the target. These will match the classes in the
    <code>deploy.jar</code> except where jars have no matching source jar.</li>
</ul>

<p>
It is good practice to use the name of the source file that is the main entry point of the
application (minus the extension). For example, if your entry point is called
<code>Main.java</code>, then your name could be <code>Main</code>.
</p>

<p>
  A <code>deps</code> attribute is not allowed in a <code>java_binary</code> rule without
  <a href="#java_binary-srcs"><code>srcs</code></a>; such a rule requires a
  <a href="#java_binary-main_class"><code>main_class</code></a> provided by
  <a href="#java_binary-runtime_deps"><code>runtime_deps</code></a>.
</p>

<p>The following code snippet illustrates a common mistake:</p>

<pre class="code">
<code class="lang-starlark">
java_binary(
    name = "DontDoThis",
    srcs = [
        <var>...</var>,
        <code class="deprecated">"GeneratedJavaFile.java"</code>,  # a generated .java file
    ],
    deps = [<code class="deprecated">":generating_rule",</code>],  # rule that generates that file
)
</code>
</pre>

<p>Do this instead:</p>

<pre class="code">
<code class="lang-starlark">
java_binary(
    name = "DoThisInstead",
    srcs = [
        <var>...</var>,
        ":generating_rule",
    ],
)
</code>
</pre>(Â
java_import¥

name(

deps

data

add_exports

	add_opens

aspect_hints

compatible_with

constraints

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties
	
exports


features

jars(


licenses

	neverlink

package_metadata

proguard_specs

restricted_to

runtime_deps

srcjar

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"ú<p>
  This rule allows the use of precompiled <code>.jar</code> files as
  libraries for <code><a href="#java_library">java_library</a></code> and
  <code>java_binary</code> rules.
</p>

<h4 id="java_import_examples">Examples</h4>

<pre class="code">
<code class="lang-starlark">
    java_import(
        name = "maven_model",
        jars = [
            "maven_model/maven-aether-provider-3.2.3.jar",
            "maven_model/maven-model-3.2.3.jar",
            "maven_model/maven-model-builder-3.2.3.jar",
        ],
    )
</code>
</pre>(‡
java_libraryï

name(

deps

srcs

data

	resources

add_exports

	add_opens

aspect_hints

bootclasspath

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties

exported_plugins
	
exports


features

javabuilder_jvm_flags

	javacopts


licenses

	neverlink

package_metadata
	
plugins

proguard_specs

resource_strip_prefix

restricted_to

runtime_deps

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"µ<p>This rule compiles and links sources into a <code>.jar</code> file.</p>

<h4>Implicit outputs</h4>
<ul>
  <li><code>lib<var>name</var>.jar</code>: A Java archive containing the class files.</li>
  <li><code>lib<var>name</var>-src.jar</code>: An archive containing the sources ("source
    jar").</li>
</ul>(≥
	java_test¬

name(

deps

srcs

data

	resources

add_exports

	add_opens

args

aspect_hints

bootclasspath

classpath_resources

compatible_with

create_executable

deploy_manifest_lines

deprecation

env

env_inherit

exec_compatible_with

exec_group_compatible_with

exec_properties


features

flaky

	javacopts

	jvm_flags


launcher


licenses

local


main_class

	neverlink

package_metadata
	
plugins

resource_strip_prefix

restricted_to

runtime_deps

shard_count

size

stamp

tags

target_compatible_with


test_class


testonly
	
timeout


toolchains

use_launcher

use_testrunner


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"ﬁ	<p>
A <code>java_test()</code> rule compiles a Java test. A test is a binary wrapper around your
test code. The test runner's main method is invoked instead of the main class being compiled.
</p>

<h4 id="java_test_implicit_outputs">Implicit output targets</h4>
<ul>
  <li><code><var>name</var>.jar</code>: A Java archive.</li>
  <li><code><var>name</var>_deploy.jar</code>: A Java archive suitable
    for deployment. (Only built if explicitly requested.) See the description of the
    <code><var>name</var>_deploy.jar</code> output from
    <a href="#java_binary">java_binary</a> for more details.</li>
</ul>

<p>
See the section on <code>java_binary()</code> arguments. This rule also
supports all <a href="https://bazel.build/reference/be/common-definitions#common-attributes-tests">attributes common
to all test rules (*_test)</a>.
</p>

<h4 id="java_test_examples">Examples</h4>

<pre class="code">
<code class="lang-starlark">

java_library(
    name = "tests",
    srcs = glob(["*.java"]),
    deps = [
        "//java/com/foo/base:testResources",
        "//java/com/foo/testing/util",
    ],
)

java_test(
    name = "AllTests",
    size = "small",
    runtime_deps = [
        ":tests",
        "//util/mysql",
    ],
)
</code>
</pre>(ì	
java_package_configurationÈ

name(

data

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

	javacopts

output_licenses

package_metadata


packages

restricted_to

system

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"Ü<p>
Configuration to apply to a set of packages.
Configurations can be added to
<code><a href="#java_toolchain.javacopts">java_toolchain.javacopts</a></code>s.
</p>

<h4 id="java_package_configuration_example">Example:</h4>

<pre class="code">
<code class="lang-starlark">

java_package_configuration(
    name = "my_configuration",
    packages = [":my_packages"],
    javacopts = ["-Werror"],
)

package_group(
    name = "my_packages",
    packages = [
        "//com/my/project/...",
        "-//com/my/project/testing/...",
    ],
)

java_toolchain(
    ...,
    package_configuration = [
        ":my_configuration",
    ]
)

</code>
</pre>(ë
java_pluginù

name(

deps

srcs

data

	resources

add_exports

	add_opens

aspect_hints

bootclasspath

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

generates_api

javabuilder_jvm_flags

	javacopts


licenses

	neverlink

output_licenses

package_metadata
	
plugins

processor_class

proguard_specs

resource_strip_prefix

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"ﬂ<p>
  <code>java_plugin</code> defines plugins for the Java compiler run by Bazel. The
  only supported kind of plugins are annotation processors. A <code>java_library</code> or
  <code>java_binary</code> rule can run plugins by depending on them via the <code>plugins</code>
  attribute. A <code>java_library</code> can also automatically export plugins to libraries that
  directly depend on it using
  <code><a href="#java_library-exported_plugins">exported_plugins</a></code>.
</p>

<h4 id="java_plugin_implicit_outputs">Implicit output targets</h4>
    <ul>
      <li><code><var>libname</var>.jar</code>: A Java archive.</li>
    </ul>

<p>Arguments are a subset of (and with identical semantics to) those of
<a href="#java_library">java_library()</a>,
except for the addition of the <code>processor_class</code> and
<code>generates_api</code> arguments.</p>(Á
java_runtimeª

name(

srcs

aspect_hints

compatible_with

default_cds

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

hermetic_srcs

hermetic_static_libs

java

	java_home


lib_ct_sym

lib_modules

output_licenses

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains
	
version


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"ñ<p>
Specifies the configuration for a Java runtime.
</p>

<h4 id="java_runtime_example">Example:</h4>

<pre class="code">
<code class="lang-starlark">

java_runtime(
    name = "jdk-9-ea+153",
    srcs = glob(["jdk9-ea+153/**"]),
    java_home = "jdk9-ea+153",
)

</code>
</pre>(Í
java_single_jarˇ

name(

deps

aspect_hints

compatible_with


compress


deploy_env

deploy_manifest_lines

deprecation

exclude_build_data

exclude_pattern

exec_compatible_with

exec_group_compatible_with

exec_properties


features

multi_release

output

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"“Collects Java dependencies and jar files into a single jar

`java_single_jar` collects Java dependencies and jar files into a single jar.
This is similar to java_binary with everything related to executables disabled,
and provides an alternative to the java_binary "deploy jar hack".

## Example

```skylark
load("//tools/build_defs/java_single_jar:java_single_jar.bzl", "java_single_jar")

java_single_jar(
    name = "my_single_jar",
    deps = [
        "//java/com/google/foo",
        "//java/com/google/bar",
    ],
)
```

Outputs:
  {name}.jar: A single jar containing all of the inputs.(’
java_toolchainø

name(

android_lint_data

android_lint_jvm_opts

android_lint_opts
$
"android_lint_package_configuration

android_lint_runner

aspect_hints

bootclasspath

compatible_javacopts

compatible_with

deprecation

deps_checker

exec_compatible_with

exec_group_compatible_with

exec_properties


features
%
#forcibly_disable_header_compilation


genclass

header_compiler
$
"header_compiler_builtin_processors

header_compiler_direct

ijar

jacocorunner

java_runtime

javabuilder

javabuilder_data

javabuilder_jvm_opts
"
 javac_supports_multiplex_workers
$
"javac_supports_worker_cancellation
,
*javac_supports_worker_multiplex_sandboxing

javac_supports_workers

	javacopts

jspecify_implicit_deps

jspecify_javacopts

jspecify_packages

jspecify_processor

jspecify_processor_class

jspecify_stubs


jvm_opts


licenses

misc


oneversion

oneversion_allowlist
 
oneversion_allowlist_for_tests

oneversion_whitelist

package_configuration

package_metadata

proguard_allowlister
+
)reduced_classpath_incompatible_processors

restricted_to

	singlejar

source_version

tags

target_compatible_with

target_version


testonly

timezone_data


toolchains

tools

turbine_data

turbine_jvm_opts


visibility

xlint

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$bzl_load_label"˛<p>
Specifies the configuration for the Java compiler. Which toolchain to be used can be changed through
the --java_toolchain argument. Normally you should not write those kind of rules unless you want to
tune your Java compiler.
</p>

<h4>Examples</h4>

<p>A simple example would be:
</p>

<pre class="code">
<code class="lang-starlark">

java_toolchain(
    name = "toolchain",
    source_version = "7",
    target_version = "7",
    bootclasspath = ["//tools/jdk:bootclasspath"],
    xlint = [ "classfile", "divzero", "empty", "options", "path" ],
    javacopts = [ "-g" ],
    javabuilder = ":JavaBuilder_deploy.jar",
)
</code>
</pre>(®
objc_import∫

name(

deps

hdrs


alwayslink

archives(

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


includes

package_metadata

restricted_to


sdk_dylibs

sdk_frameworks

sdk_includes

tags

target_compatible_with


testonly

textual_hdrs


toolchains


visibility

weak_sdk_frameworks

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"Ÿ<p>This rule encapsulates an already-compiled static library in the form of an
<code>.a</code> file. It also allows exporting headers and resources using the same
attributes supported by <code>objc_library</code>.</p>(¬
objc_library⁄

name(

deps

srcs

data

hdrs


alwayslink

aspect_hints

compatible_with

	conlyopts

copts
	
cxxopts
	
defines

deprecation

enable_modules

exec_compatible_with

exec_group_compatible_with

exec_properties


features

implementation_deps


includes


linkopts


module_map

module_name

non_arc_srcs

package_metadata

pch

restricted_to


sdk_dylibs

sdk_frameworks

sdk_includes

stamp

tags

target_compatible_with


testonly

textual_hdrs


toolchains


visibility

weak_sdk_frameworks

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


licenses


distribs

$bzl_load_label"S<p>This rule produces a static library from the given Objective-C source files.</p>(Ú
cc_proto_libraryà

name(

deps

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"–<p>
<code>cc_proto_library</code> generates C++ code from <code>.proto</code> files.
</p>

<p>
<code>deps</code> must point to <a href="protocol-buffer.html#proto_library"><code>proto_library
</code></a> rules.
</p>

<p>
Example:
</p>

<pre>
<code class="lang-starlark">
cc_library(
    name = "lib",
    deps = [":foo_cc_proto"],
)

cc_proto_library(
    name = "foo_cc_proto",
    deps = [":foo_proto"],
)

proto_library(
    name = "foo_proto",
)
</code>
</pre>(Å
java_lite_proto_libraryà

name(

deps

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"ÿ<p>
<code>java_lite_proto_library</code> generates Java code from <code>.proto</code> files.
</p>

<p>
<code>deps</code> must point to <a href="protocol-buffer.html#proto_library"><code>proto_library
</code></a> rules.
</p>

<p>
Example:
</p>

<pre class="code">
<code class="lang-starlark">
java_library(
    name = "lib",
    runtime_deps = [":foo"],
)

java_lite_proto_library(
    name = "foo",
    deps = [":bar"],
)

proto_library(
    name = "bar",
)
</code>
</pre>(†
java_proto_libraryî

name(

deps

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


licenses

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"<p>
<code>java_proto_library</code> generates Java code from <code>.proto</code> files.
</p>

<p>
<code>deps</code> must point to <a href="protocol-buffer.html#proto_library"><code>proto_library
</code></a> rules.
</p>

<p>
Example:
</p>

<pre class="code">
<code class="lang-starlark">
java_library(
    name = "lib",
    runtime_deps = [":foo_java_proto"],
)

java_proto_library(
    name = "foo_java_proto",
    deps = [":foo_proto"],
)

proto_library(
    name = "foo_proto",
)
</code>
</pre>(·
proto_libraryë

name(

deps

srcs

data

allow_exports

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties
	
exports

extension_declarations


features

import_prefix


licenses

option_deps

package_metadata

restricted_to

strip_import_prefix

tags

target_compatible_with


testonly


toolchains


visibility"π<p>Use <code>proto_library</code> to define libraries of protocol buffers which
may be used from multiple languages. A <code>proto_library</code> may be listed
in the <code>deps</code> clause of supported rules, such as
<code>java_proto_library</code>.

<p>When compiled on the command-line, a <code>proto_library</code> creates a file
named <code>foo-descriptor-set.proto.bin</code>, which is the descriptor set for
the messages the rule srcs. The file is a serialized
<code>FileDescriptorSet</code>, which is described in
<a href="https://developers.google.com/protocol-buffers/docs/techniques#self-description">
https://developers.google.com/protocol-buffers/docs/techniques#self-description</a>.

<p>It only contains information about the <code>.proto</code> files directly
mentioned by a <code>proto_library</code> rule; the collection of transitive
descriptor sets is available through the
<code>[ProtoInfo].transitive_descriptor_sets</code> Starlark provider.
See documentation in <code>proto_info.bzl</code>.

<p>Recommended code organization:
<ul>
<li>One <code>proto_library</code> rule per <code>.proto</code> file.
<li>A file named <code>foo.proto</code> will be in a rule named <code>foo_proto</code>,
  which is located in the same package.
<li>A <code>[language]_proto_library</code> that wraps a <code>proto_library</code>
  named <code>foo_proto</code> should be called <code>foo_[language]_proto</code>,
  and be located in the same package.
</ul>(ï
py_proto_libraryà

name(

deps

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"Û      Use `py_proto_library` to generate Python libraries from `.proto` files.

      The convention is to name the `py_proto_library` rule `foo_py_pb2`,
      when it is wrapping `proto_library` rule `foo_proto`.

      `deps` must point to a `proto_library` rule.

      Example:

```starlark
py_library(
    name = "lib",
    deps = [":foo_py_pb2"],
)

py_proto_library(
    name = "foo_py_pb2",
    deps = [":foo_proto"],
)

proto_library(
    name = "foo_proto",
    srcs = ["foo.proto"],
)
```(Ê
proto_lang_toolchainÊ

name(

allowlist_different_package

aspect_hints

blacklisted_protos

command_line(

compatible_with

denylisted_protos

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


mnemonic

output_files

package_metadata

plugin

plugin_format_flag

progress_message

protoc_minimal_do_not_use

restricted_to
	
runtime

tags

target_compatible_with


testonly

toolchain_type


toolchains


visibility"‚<p>If using Bazel, please load the rule from <a href="https://github.com/bazelbuild/rules_proto">
https://github.com/bazelbuild/rules_proto</a>.

<p>Specifies how a LANG_proto_library rule (e.g., <code>java_proto_library</code>) should invoke the
proto-compiler.
Some LANG_proto_library rules allow specifying which toolchain to use using command-line flags;
consult their documentation.

<p>Normally you should not write those kind of rules unless you want to
tune your Java compiler.

<p>There's no compiler. The proto-compiler is taken from the proto_library rule we attach to. It is
passed as a command-line flag to Blaze.
Several features require a proto-compiler to be invoked on the proto_library rule itself.
It's beneficial to enforce the compiler that LANG_proto_library uses is the same as the one
<code>proto_library</code> does.

<h4>Examples</h4>

<p>A simple example would be:
<pre><code class="lang-starlark">
proto_lang_toolchain(
    name = "javalite_toolchain",
    command_line = "--javalite_out=shared,immutable:$(OUT)",
    plugin = ":javalite_plugin",
    runtime = ":protobuf_lite",
)
</code></pre>(Ë
proto_toolchain“

name(

aspect_hints

command_line

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


mnemonic

output_files

package_metadata

progress_message

proto_compiler

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility(Ì
	py_binary›

name(

deps

srcs

data

args

aspect_hints

compatible_with

deprecation


distribs

env

exec_compatible_with

exec_group_compatible_with

exec_properties


features
	
imports

interpreter_args

legacy_create_init


licenses

main

main_module

output_licenses

package_metadata


precompile

precompile_invalidation_mode

precompile_optimize_level

precompile_source_retention

pyc_collection


pyi_deps


pyi_srcs

python_version

restricted_to

srcs_version

stamp

tags

target_compatible_with


testonly


toolchains


visibility(ó

py_libraryÒ

name(

deps

srcs

data

aspect_hints

compatible_with

deprecation


distribs

exec_compatible_with

exec_group_compatible_with

exec_properties
"
 experimental_venvs_site_packages


features
	
imports


licenses

package_metadata


precompile

precompile_invalidation_mode

precompile_optimize_level

precompile_source_retention


pyi_deps


pyi_srcs

restricted_to

srcs_version

tags

target_compatible_with


testonly


toolchains


visibility"íA library of Python code that can be depended upon.

Default outputs:
* The input Python sources
* The precompiled artifacts from the sources.

NOTE: Precompilation affects which of the default outputs are included in the
resulting runfiles. See the precompile-related attributes and flags for
more information.

:::{versionchanged} 0.37.0
Source files are no longer added to the runfiles directly.
:::(õ
py_testç

name(

deps

srcs

data

args

aspect_hints

compatible_with

deprecation


distribs

env

env_inherit

exec_compatible_with

exec_group_compatible_with

exec_properties


features

flaky
	
imports

interpreter_args

legacy_create_init


licenses

local

main

main_module

package_metadata


precompile

precompile_invalidation_mode

precompile_optimize_level

precompile_source_retention

pyc_collection


pyi_deps


pyi_srcs

python_version

restricted_to

shard_count

size

srcs_version

stamp

tags

target_compatible_with


testonly
	
timeout


toolchains


visibility(ª

py_runtime§

name(

	abi_flags

aspect_hints

bootstrap_template

compatible_with

coverage_tool

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

files

implementation_name

interpreter

interpreter_path

interpreter_version_info

package_metadata
	
pyc_tag

python_version

restricted_to

site_init_template

stage2_bootstrap_template

stub_shebang

supports_build_time_venv

tags

target_compatible_with


testonly


toolchains


visibility

zip_main_template"É	Represents a Python runtime used to execute Python code.

A `py_runtime` target can represent either a *platform runtime* or an *in-build
runtime*. A platform runtime accesses a system-installed interpreter at a known
path, whereas an in-build runtime points to an executable target that acts as
the interpreter. In both cases, an "interpreter" means any executable binary or
wrapper script that is capable of running a Python script passed on the command
line, following the same conventions as the standard CPython interpreter.

A platform runtime is by its nature non-hermetic. It imposes a requirement on
the target platform to have an interpreter located at a specific path. An
in-build runtime may or may not be hermetic, depending on whether it points to
a checked-in interpreter or a wrapper script that accesses the system
interpreter.

Example

```
load("@rules_python//python:py_runtime.bzl", "py_runtime")

py_runtime(
    name = "python-2.7.12",
    files = glob(["python-2.7.12/**"]),
    interpreter = "python-2.7.12/bin/python",
)

py_runtime(
    name = "python-3.6.0",
    interpreter_path = "/opt/pyenv/versions/3.6.0/bin/python",
)
```(Ø	
	sh_binaryﬁ

name(

deps

srcs

data

args

aspect_hints

compatible_with

deprecation

env

env_inherit

exec_compatible_with

exec_group_compatible_with

exec_properties


features

output_licenses

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains

use_bash_launcher


visibility"æ<p>
  The <code>sh_binary</code> rule is used to declare executable shell scripts.
  (<code>sh_binary</code> is a misnomer: its outputs aren't necessarily binaries.) This rule ensures
  that all dependencies are built, and appear in the <code>runfiles</code> area at execution time.
  We recommend that you name your <code>sh_binary()</code> rules after the name of the script minus
  the extension (e.g. <code>.sh</code>); the rule name and the file name must be distinct.
  <code>sh_binary</code> respects shebangs, so any available interpreter may be used (eg.
  <code>#!/bin/zsh</code>)
</p>
<h4 id="sh_binary_examples">Example</h4>
<p>For a simple shell script with no dependencies and some data files:
</p>
<pre class="code">
sh_binary(
    name = "foo",
    srcs = ["foo.sh"],
    data = glob(["datafiles/*.txt"]),
)
</pre>(Ò

sh_libraryò

name(

deps

srcs

data

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains


visibility"≈
<p>
  The main use for this rule is to aggregate together a logical
  "library" consisting of related scripts&mdash;programs in an
  interpreted language that does not require compilation or linking,
  such as the Bourne shell&mdash;and any data those programs need at
  run-time. Such "libraries" can then be used from
  the <code>data</code> attribute of one or
  more <code>sh_binary</code> rules.
</p>

<p>
  You can use the <a href="#filegroup"><code>filegroup</code></a> rule to aggregate data
  files.
</p>

<p>
  In interpreted programming languages, there's not always a clear
  distinction between "code" and "data": after all, the program is
  just "data" from the interpreter's point of view. For this reason
  this rule has three attributes which are all essentially equivalent:
  <code>srcs</code>, <code>deps</code> and <code>data</code>.
  The current implementation does not distinguish between the elements of these lists.
  All three attributes accept rules, source files and generated files.
  It is however good practice to use the attributes for their usual purpose (as with other rules).
</p>

<h4 id="sh_library_examples">Examples</h4>

<pre class="code">
sh_library(
    name = "foo",
    data = [
        ":foo_service_script",  # an sh_binary with srcs
        ":deploy_foo",  # another sh_binary with srcs
    ],
)
</pre>(∫
sh_testˇ

name(

deps

srcs

data

args

aspect_hints

compatible_with

deprecation

env

env_inherit

exec_compatible_with

exec_group_compatible_with

exec_properties


features

flaky

local

package_metadata

restricted_to

shard_count

size

tags

target_compatible_with


testonly
	
timeout


toolchains

use_bash_launcher


visibility"™<p>A <code>sh_test()</code> rule creates a test written as a Bourne shell script.</p>

<p>See the <a href="#common-attributes-tests">
attributes common to all test rules (*_test)</a>.</p>

<h4 id="sh_test_examples">Examples</h4>

<pre class="code">
sh_test(
    name = "foo_integration_test",
    size = "small",
    srcs = ["foo_integration_test.sh"],
    deps = [":foo_sh_lib"],
    data = glob(["testdata/*.txt"]),
)
</pre>(ü
action_listenerò

name(

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties

extra_actions(


features


licenses

	mnemonics(

package_metadata

restricted_to

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs"Ó

<p>
  <b>WARNING:</b> Extra actions are deprecated. Use
  <a href="https://bazel.build/rules/aspects">aspects</a>
  instead.
</p>

<p>
  An <code>action_listener</code> rule doesn't produce any output itself.
  Instead, it allows tool developers to insert
  <a href="#extra_action"><code>extra_action</code></a>s into the build system,
  by providing a mapping from action to <a href="#extra_action"><code>extra_action</code>
  </a>.
</p>

<p>
  This rule's arguments map action mnemonics to
  <a href="#extra_action"><code>extra_action</code></a> rules.
</p>

<p>
  By specifying the option <a href="/docs/user-manual#flag--experimental_action_listener">
  <code>--experimental_action_listener=&lt;label&gt;</code></a>,
  the build will use the specified <code>action_listener</code> to insert
  <a href="#extra_action"><code>extra_action</code></a>s into the build graph.
</p>

<h4 id="action_listener_example">Example</h4>
<pre>
action_listener(
    name = "index_all_languages",
    mnemonics = [
        "Javac",
        "CppCompile",
        "Python",
    ],
    extra_actions = [":indexer"],
)

action_listener(
    name = "index_java",
    mnemonics = ["Javac"],
    extra_actions = [":indexer"],
)

extra_action(
    name = "indexer",
    tools = ["//my/tools:indexer"],
    cmd = "$(location //my/tools:indexer)" +
          "--extra_action_file=$(EXTRA_ACTION_FILE)",
)
</pre>

(†
extra_action…

name(

data

aspect_hints

cmd(

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


licenses

out_templates

package_metadata

requires_action_output

restricted_to

tags

target_compatible_with


testonly


toolchains

tools


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs"¡
<p>
  <b>WARNING:</b> Extra actions are deprecated. Use
  <a href="https://bazel.build/rules/aspects">aspects</a>
  instead.
</p>

<p>
  An <code>extra_action</code> rule doesn't produce any meaningful output
  when specified as a regular build target. Instead, it allows tool developers
  to insert additional actions into the build graph that shadow existing actions.
</p>

<p>
  See <a href="#action_listener"><code>action_listener</code></a> for details
  on how to enable <code>extra_action</code>s.
</p>

<p>
  The <code>extra_action</code>s run as a command-line. The command-line tool gets
  access to a file containing a protocol buffer as $(EXTRA_ACTION_FILE)
  with detailed information on the original action it is shadowing.
  It also has access to all the input files the original action has access to.
  See <tt>extra_actions_base.proto</tt>
  for details on the data stored inside the protocol buffer. Each proto file
  contains an ExtraActionInfo message.
</p>

<p>
  Just like all other actions, extra actions are sandboxed, and should be designed to handle that.
</p>

(ê
alias°

name(


actual(

aspect_hints

compatible_with

deprecation


features

package_metadata

restricted_to

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies"‡
<p>
  The <code>alias</code> rule creates another name a rule can be referred to as.
</p>

<p>
  Aliasing only works for "regular" targets. In particular, <code>package_group</code>
  and <code>test_suite</code> cannot be aliased.
</p>

<p>
  Aliasing may be of help in large repositories where renaming a target would require making
  changes to lots of files. You can also use alias rule to store a
  <a href="#select">select</a> function call if you want to reuse that logic for
  multiple targets.
</p>

<p>
  The alias rule has its own visibility declaration. In all other respects, it behaves
  like the rule it references (e.g. testonly <em>on the alias</em> is ignored; the testonly-ness
   of the referenced rule is used instead) with some minor exceptions:

  <ul>
    <li>
      Tests are not run if their alias is mentioned on the command line. To define an alias
      that runs the referenced test, use a <a href="#test_suite"><code>test_suite</code></a>
      rule with a single target in its <a href="#test_suite.tests"><code>tests</code></a>
      attribute.
    </li>
    <li>
      When defining environment groups, the aliases to <code>environment</code> rules are not
      supported. They are not supported in the <code>--target_environment</code> command line
      option, either.
    </li>
  </ul>
</p>

<h4 id="alias_example">Examples</h4>

<pre class="code">
filegroup(
    name = "data",
    srcs = ["data.txt"],
)

alias(
    name = "other",
    actual = ":data",
)
</pre>

(…*
config_setting⁄

name(

aspect_hints

constraint_values

define_values

deprecation


features

flag_values


licenses

package_metadata

tags


testonly

values


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

:flag_alias_settings"◊'
  <p>
    Matches an expected configuration state (expressed as build flags or platform constraints) for
    the purpose of triggering configurable attributes. See <a href="#select">select</a> for
    how to consume this rule and <a href="#configurable-attributes">
    Configurable attributes</a> for an overview of the general feature.

  <h4 id="config_setting_examples">Examples</h4>

  <p>The following matches any build that sets <code>--compilation_mode=opt</code> or
  <code>-c opt</code> (either explicitly at the command line or implicitly from .bazelrc files):
  </p>

  <pre class="code">
  config_setting(
      name = "simple",
      values = {"compilation_mode": "opt"}
  )
  </pre>

  <p>The following matches any build that targets ARM and applies the custom define
  <code>FOO=bar</code> (for instance, <code>bazel build --cpu=arm --define FOO=bar ...</code>):
  </p>

  <pre class="code">
  config_setting(
      name = "two_conditions",
      values = {
          "cpu": "arm",
          "define": "FOO=bar"
      }
  )
  </pre>

  <p>The following matches any build that sets
     <a href="https://bazel.build/rules/config#user-defined-build-settings">user-defined flag</a>
     <code>--//custom_flags:foo=1</code> (either explicitly at the command line or implicitly from
     .bazelrc files):
  </p>

  <pre class="code">
  config_setting(
      name = "my_custom_flag_is_set",
      flag_values = { "//custom_flags:foo": "1" },
  )
  </pre>

  <p>The following matches any build that targets a platform with an x86_64 architecture and glibc
     version 2.25, assuming the existence of a <code>constraint_value</code> with label
     <code>//example:glibc_2_25</code>. Note that a platform still matches if it defines additional
     constraint values beyond these two.
  </p>

  <pre class="code">
  config_setting(
      name = "64bit_glibc_2_25",
      constraint_values = [
          "@platforms//cpu:x86_64",
          "//example:glibc_2_25",
      ]
  )
  </pre>

  In all these cases, it's possible for the configuration to change within the build, for example if
  a target needs to be built for a different platform than its dep. This means that even when a
  <code>config_setting</code> doesn't match the top-level command-line flags, it may still match
  some build targets.

  <h4 id="config_setting_notes">Notes</h4>
  <ul>
    <li>See <a href="#select">select</a> for what happens when multiple
       <code>config_setting</code>s match the current configuration state.
    </li>

    <li>For flags that support shorthand forms (e.g. <code>--compilation_mode</code> vs.
      <code>-c</code>), <code>values</code> definitions must use the full form. These automatically
      match invocations using either form.
    </li>

    <li>
      If a flag takes multiple values (like <code>--copt=-Da --copt=-Db</code> or a list-typed
      <a href="https://bazel.build/rules/config#user-defined-build-settings">
      Starlark flag</a>), <code>values = { "flag": "a" }</code> matches if <code>"a"</code> is
      present <i>anywhere</i> in the actual list.

      <p>
        <code>values = { "myflag": "a,b" }</code> works the same way: this matches
        <code>--myflag=a --myflag=b</code>, <code>--myflag=a --myflag=b --myflag=c</code>,
        <code>--myflag=a,b</code>, and <code>--myflag=c,b,a</code>. Exact semantics vary between
        flags. For example, <code>--copt</code> doesn't support multiple values <i>in the same
        instance</i>: <code>--copt=a,b</code> produces <code>["a,b"]</code> while <code>--copt=a
        --copt=b</code> produces <code>["a", "b"]</code> (so <code>values = { "copt": "a,b" }</code>
        matches the former but not the latter). But <code>--ios_multi_cpus</code> (for Apple rules)
        <i>does</i>: <code>-ios_multi_cpus=a,b</code> and <code>ios_multi_cpus=a --ios_multi_cpus=b
        </code> both produce <code>["a", "b"]</code>. Check flag definitions and test your
        conditions carefully to verify exact expectations.
      </p>
    </li>

    <li>If you need to define conditions that aren't modeled by built-in build flags, use
      <a href="https://bazel.build/rules/config#user-defined-build-settings">
      Starlark-defined flags</a>. You can also use <code>--define</code>, but this offers weaker
      support and is not recommended. See
      <a href="#configurable-attributes">here</a> for more discussion.
    </li>

    <li>Avoid repeating identical <code>config_setting</code> definitions in different packages.
      Instead, reference a common <code>config_setting</code> that defined in a canonical package.
    </li>

    <li><a href="general.html#config_setting.values"><code>values</code></a>,
       <a href="general.html#config_setting.define_values"><code>define_values</code></a>, and
       <a href="general.html#config_setting.constraint_values"><code>constraint_values</code></a>
       can be used in any combination in the same <code>config_setting</code> but at least one must
       be set for any given <code>config_setting</code>.
    </li>
  </ul>
(„
	filegroupÙ

name(

srcs

data

aspect_hints

compatible_with

deprecation


features


licenses

output_group

package_metadata

restricted_to

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

output_licenses"‹<p>
  Use <code>filegroup</code> to gather the outputs of a set of targets under a single
  label.
</p>

<p>
  <code>filegroup</code> is not a substitute for listing targets on the command line or
  in an attribute of another rule, because targets have many properties other than their
  outputs, which are not collected in the same way. However, it's still useful in quite
  a few cases, for example, in the <code>srcs</code> attribute of a genrule, or
  the <code>data</code> attribute of a *_binary rule.
</p>

<p>
  Using <code>filegroup</code> is encouraged instead of referencing directories directly.
  Directly referencing directories is discouraged because the build system does not have
  full knowledge of all files below the directory, so it may not rebuild when these files change.
  When combined with <a href="#glob">glob</a>, <code>filegroup</code> can ensure that all
  files are explicitly known to the build system.
</p>

<h4 id="filegroup_example">Examples</h4>

<p>
  To create a <code>filegroup</code> consisting of two source files, do
</p>
<pre class="code">
filegroup(
    name = "mygroup",
    srcs = [
        "a_file.txt",
        "//a/library:target",
        "//a/binary:target",
    ],
)
</pre>
<p>
  Or, use a <code>glob</code> to fully crawl a testdata directory:
</p>
<pre class="code">
filegroup(
    name = "exported_testdata",
    srcs = glob([
        "testdata/*.dat",
        "testdata/logs/**&#47;*.log",
    ]),
)
</pre>
<p>
  To make use of these definitions, reference the <code>filegroup</code> with a label from any rule:
</p>
<pre class="code">
cc_library(
    name = "my_library",
    srcs = ["foo.cc"],
    data = [
        "//my_package:exported_testdata",
        "//my_package:mygroup",
    ],
)
</pre>

(ƒ
genquery‹

name(

deps

data

aspect_hints

compatible_with

compressed_output

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


expression(


features


licenses

opts

package_metadata

restricted_to
	
scope(

strict

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs"÷
  <p>
  <code>genquery()</code> runs a query specified in the
    <a href="/reference/query">Bazel query language</a> and dumps the result
    into a file.
  </p>
  <p>
    In order to keep the build consistent, the query is allowed only to visit
    the transitive closure of the targets specified in the <code>scope</code>
    attribute. Queries violating this rule will fail during execution if
    <code>strict</code> is unspecified or true (if <code>strict</code> is false,
    the out of scope targets will simply be skipped with a warning). The
    easiest way to make sure this does not happen is to mention the same labels
    in the scope as in the query expression.
  </p>
  <p>
    The only difference between the queries allowed here and on the command
    line is that queries containing wildcard target specifications (e.g.
    <code>//pkg:*</code> or <code>//pkg:all</code>) are not allowed here.
    The reasons for this are two-fold: first, because <code>genquery</code> has
    to specify a scope to prevent targets outside the transitive closure of the
    query to influence its output; and, second, because <code>BUILD</code> files
    do not support wildcard dependencies (e.g. <code>deps=["//a/..."]</code>
    is not allowed).
  </p>
  <p>
    The genquery's output is ordered lexicographically in order to enforce deterministic output,
    with the exception of <code>--output=graph|minrank|maxrank</code> or when <code>somepath</code>
    is used as the top-level function.
  <p>
    The name of the output file is the name of the rule.
  </p>

<h4 id="genquery_examples">Examples</h4>
  <p>
    This example writes the list of the labels in the transitive closure of the
    specified target to a file.
  </p>

<pre class="code">
genquery(
    name = "kiwi-deps",
    expression = "deps(//kiwi:kiwi_lib)",
    scope = ["//kiwi:kiwi_lib"],
)
</pre>

(åK
genruleÓ

name(

srcs

outs(

aspect_hints

cmd


cmd_bash
	
cmd_bat

cmd_ps

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


executable


features


licenses

local
	
message

output_licenses

output_to_bindir

package_metadata

restricted_to

tags

target_compatible_with


testonly


toolchains

tools


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$is_executable

heuristic_label_expansion

$genrule_setup

stamp"çF
<p>A <code>genrule</code> generates one or more files using a user-defined Bash command.</p>

<p>
  Genrules are generic build rules that you can use if there's no specific rule for the task.
  For example, you could run a Bash one-liner. If however you need to compile C++ files, stick
  to the existing <code>cc_*</code> rules, because all the heavy lifting has already been done
  for you.
</p>
<p>
  Note that genrule requires a shell to interpret the command argument.
  It is also easy to reference arbitrary programs available on the PATH, however this makes the
  command non-hermetic and may not be reproducible.
  If you only need to run a single tool, consider using
  <a href="https://github.com/bazelbuild/bazel-skylib/blob/main/docs/run_binary_doc.md">run_binary</a>
  instead.
</p>
<p>
  Like every other action, the action created by genrules should not assume anything about their
  working directory; all Bazel guarantees is that their declared inputs will be available at the
  path that <code>$(location)</code> returns for their label. For example, if the action is run in a
  sandbox or remotely, the implementation of the sandbox or the remote execution will determine the
  working directory. If run directly (using the <code>standalone</code> strategy), the working
  directory will be the execution root, i.e. the result of <code>bazel info execution_root</code>.
</p>
<p>
  Do not use a genrule for running tests. There are special dispensations for tests and test
  results, including caching policies and environment variables. Tests generally need to be run
  after the build is complete and on the target architecture, whereas genrules are executed during
  the build and on the exec architecture (the two may be different). If you need a general purpose
  testing rule, use <a href="#sh_test"><code>sh_test</code></a>.
</p>

<h4>Cross-compilation Considerations</h4>

<p>
  <em>See <a href="/docs/user-manual#configurations">the user manual</a> for more info about
  cross-compilation.</em>
</p>
<p>
  While genrules run during a build, their outputs are often used after the build, for deployment or
  testing. Consider the example of compiling C code for a microcontroller: the compiler accepts C
  source files and generates code that runs on a microcontroller. The generated code obviously
  cannot run on the CPU that was used for building it, but the C compiler (if compiled from source)
  itself has to.
</p>
<p>
  The build system uses the exec configuration to describe the machine(s) on which the build runs
  and the target configuration to describe the machine(s) on which the output of the build is
  supposed to run. It provides options to configure each of these and it segregates the
  corresponding files into separate directories to avoid conflicts.
</p>
<p>
  For genrules, the build system ensures that dependencies are built appropriately:
  <code>srcs</code> are built (if necessary) for the <em>target</em> configuration,
  <code>tools</code> are built for the <em>exec</em> configuration, and the output is considered to
  be for the <em>target</em> configuration. It also provides <a href="#make-variables">
  "Make" variables</a> that genrule commands can pass to the corresponding tools.
</p>
<p>
  It is intentional that genrule defines no <code>deps</code> attribute: other built-in rules use
  language-dependent meta information passed between the rules to automatically determine how to
  handle dependent rules, but this level of automation is not possible for genrules. Genrules work
  purely at the file and runfiles level.
</p>

<h4>Special Cases</h4>

<p>
  <i>Exec-exec compilation</i>: in some cases, the build system needs to run genrules such that the
  output can also be executed during the build. If for example a genrule builds some custom compiler
  which is subsequently used by another genrule, the first one has to produce its output for the
  exec configuration, because that's where the compiler will run in the other genrule. In this case,
  the build system does the right thing automatically: it builds the <code>srcs</code> and
  <code>outs</code> of the first genrule for the exec configuration instead of the target
  configuration. See <a href="/docs/user-manual#configurations">the user manual</a> for more
  info.
</p>
<p>
  <i>JDK & C++ Tooling</i>: to use a tool from the JDK or the C++ compiler suite, the build system
  provides a set of variables to use. See <a href="#make-variables">"Make" variable</a> for
  details.
</p>

<h4>Genrule Environment</h4>

<p>
  The genrule command is executed by a Bash shell that is configured to fail when a command
  or a pipeline fails, using <code>set -e -o pipefail</code>.
</p>
<p>
  The build tool executes the Bash command in a sanitized process environment that
  defines only core variables such as <code>PATH</code>, <code>PWD</code>,
  <code>TMPDIR</code>, and a few others.

  To ensure that builds are reproducible, most variables defined in the user's shell
  environment are not passed though to the genrule's command. However, Bazel (but not
  Blaze) passes through the value of the user's <code>PATH</code> environment variable.

  Any change to the value of <code>PATH</code> will cause Bazel to re-execute the command
  on the next build.
  <!-- See https://github.com/bazelbuild/bazel/issues/1142 -->
</p>
<p>
  A genrule command should not access the network except to connect processes that are
  children of the command itself, though this is not currently enforced.
</p>
<p>
  The build system automatically deletes any existing output files, but creates any necessary parent
  directories before it runs a genrule. It also removes any output files in case of a failure.
</p>

<h4>General Advice</h4>

<ul>
  <li>Do ensure that tools run by a genrule are deterministic and hermetic. They should not write
    timestamps to their output, and they should use stable ordering for sets and maps, as well as
    write only relative file paths to the output, no absolute paths. Not following this rule will
    lead to unexpected build behavior (Bazel not rebuilding a genrule you thought it would) and
    degrade cache performance.</li>
  <li>Do use <code>$(location)</code> extensively, for outputs, tools and sources. Due to the
    segregation of output files for different configurations, genrules cannot rely on hard-coded
    and/or absolute paths.</li>
  <li>Do write a common Starlark macro in case the same or very similar genrules are used in
    multiple places. If the genrule is complex, consider implementing it in a script or as a
    Starlark rule. This improves readability as well as testability.</li>
  <li>Do make sure that the exit code correctly indicates success or failure of the genrule.</li>
  <li>Do not write informational messages to stdout or stderr. While useful for debugging, this can
    easily become noise; a successful genrule should be silent. On the other hand, a failing genrule
    should emit good error messages.</li>
  <li><code>$$</code> evaluates to a <code>$</code>, a literal dollar-sign, so in order to invoke a
    shell command containing dollar-signs such as <code>ls $(dirname $x)</code>, one must escape it
    thus: <code>ls $$(dirname $$x)</code>.</li>
  <li>Avoid creating symlinks and directories. Bazel doesn't copy over the directory/symlink
    structure created by genrules and its dependency checking of directories is unsound.</li>
  <li>When referencing the genrule in other rules, you can use either the genrule's label or the
    labels of individual output files. Sometimes the one approach is more readable, sometimes the
    other: referencing outputs by name in a consuming rule's <code>srcs</code> will avoid
    unintentionally picking up other outputs of the genrule, but can be tedious if the genrule
    produces many outputs.</li>
</ul>

<h4 id="genrule_examples">Examples</h4>

<p>
  This example generates <code>foo.h</code>. There are no sources, because the command doesn't take
  any input. The "binary" run by the command is a perl script in the same package as the genrule.
</p>
<pre class="code">
genrule(
    name = "foo",
    srcs = [],
    outs = ["foo.h"],
    cmd = "./$(location create_foo.pl) &gt; \"$@\"",
    tools = ["create_foo.pl"],
)
</pre>

<p>
  The following example shows how to use a <a href="#filegroup"><code>filegroup</code>
  </a> and the outputs of another <code>genrule</code>. Note that using <code>$(SRCS)</code> instead
  of explicit <code>$(location)</code> directives would also work; this example uses the latter for
  sake of demonstration.
</p>
<pre class="code">
genrule(
    name = "concat_all_files",
    srcs = [
        "//some:files",  # a filegroup with multiple files in it ==> $(location<b>s</b>)
        "//other:gen",   # a genrule with a single output ==> $(location)
    ],
    outs = ["concatenated.txt"],
    cmd = "cat $(locations //some:files) $(location //other:gen) > $@",
)
</pre>

(˘
starlark_doc_extractÈ

name(

deps

src(

data

allow_unused_doc_comments

aspect_hints

compatible_with

deprecation

exec_compatible_with

exec_group_compatible_with

exec_properties


features


licenses

package_metadata

render_main_repo_name

restricted_to

symbol_names

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs"Ú	
<p><code>starlark_doc_extract()</code> extracts documentation for rules, functions (including
macros), aspects, and providers defined or re-exported in a given <code>.bzl</code> or
<code>.scl</code> file. The output of this rule is a <code>ModuleInfo</code> binary proto as defined
in
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/protobuf/stardoc_output.proto">stardoc_output.proto</a>
in the Bazel source tree.

<h4 id="starlark_doc_extract_implicit_outputs">Implicit output targets</h4>
        <ul>
          <li><code><var>name</var>.binaryproto</code> (the default output): A
            <code>ModuleInfo</code> binary proto.</li>
          <li><code><var>name</var>.textproto</code> (only built if explicitly requested): the text
            proto version of <code><var>name</var>.binaryproto</code>.</li>
        </ul>


Note: the exact output of this rule is not a stable public API. For example, the set of
natively-defined common rule attributes and their docstrings may change even with minor Bazel
releases. For this reason, documentation generated for user-defined rules is not stable across Bazel
releases, so we suggest taking care that any "golden tests" based on outputs of this rule are only
run with a single Bazel version.

(Ú

test_suite›

name(

aspect_hints

compatible_with

deprecation


features


licenses

package_metadata

restricted_to

tags

target_compatible_with


testonly

tests


visibility

transitive_configs

generator_name

generator_function

generator_location

:action_listener

$config_dependencies


distribs

$implicit_tests"Å	
<p>
A <code>test_suite</code> defines a set of tests that are considered "useful" to humans. This
allows projects to define sets of tests, such as "tests you must run before checkin", "our
project's stress tests" or "all small tests." The <code>bazel test</code> command respects this sort
of organization: For an invocation like <code>bazel test //some/test:suite</code>, Bazel first
enumerates all test targets transitively included by the <code>//some/test:suite</code> target (we
call this "test_suite expansion"), then Bazel builds and tests those targets.
</p>

<h4 id="test_suite_examples">Examples</h4>

<p>A test suite to run all of the small tests in the current package.</p>
<pre class="code">
test_suite(
    name = "small_tests",
    tags = ["small"],
)
</pre>

<p>A test suite that runs a specified set of tests:</p>

<pre class="code">
test_suite(
    name = "smoke_tests",
    tests = [
        "system_unittest",
        "public_api_unittest",
    ],
)
</pre>

<p>A test suite to run all tests in the current package which are not flaky.</p>
<pre class="code">
test_suite(
    name = "non_flaky_test",
    tags = ["-flaky"],
)
</pre>

(Ø
constraint_setting˜

name(

aspect_hints

default_constraint_value

deprecation


features


licenses

tags


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs"ú
<p>This rule is used to introduce a new constraint type for which a platform may specify a value.
For instance, you might define a <code>constraint_setting</code> named "glibc_version" to represent
the capability for platforms to have different versions of the glibc library installed.

For more details, see the
<a href="https://bazel.build/docs/platforms">Platforms</a> page.

<p>Each <code>constraint_setting</code> has an extensible set of associated
<code>constraint_value</code>s. Usually these are defined in the same package, but sometimes a
different package will introduce new values for an existing setting. For instance, the predefined
setting <code>@platforms//cpu:cpu</code> can be extended with a custom value in order to
define a platform targeting an obscure cpu architecture.

(ﬁ
constraint_valueÛ

name(

aspect_hints

constraint_setting(

deprecation


features


licenses

tags


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs"—
This rule introduces a new value for a given constraint type.

For more details, see the
<a href="https://bazel.build/docs/platforms">Platforms</a> page.

<h4 id="constraint_value_examples">Example</h4>
<p>The following creates a new possible value for the predefined <code>constraint_value</code>
representing cpu architecture.
<pre class="code">
constraint_value(
    name = "mips",
    constraint_setting = "@platforms//cpu:cpu",
)
</pre>

Platforms can then declare that they have the <code>mips</code> architecture as an alternative to
<code>x86_64</code>, <code>arm</code>, and so on.

(∂3
platform˚

name(

aspect_hints

constraint_values

deprecation

exec_properties


features

flags


licenses

missing_toolchain_error
	
parents

required_settings

tags


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs

check_toolchain_types

allowed_toolchain_types"©0
<p>This rule defines a new platform -- a named collection of constraint choices
(such as cpu architecture or compiler version) describing an environment in
which part of the build may run.

For more details, see the <a href="/extending/platforms">Platforms</a> page.


<h4 id="platform_examples">Example</h4>
<p>
  This defines a platform that describes any environment running Linux on ARM.
</p>
<pre class="code">
platform(
    name = "linux_arm",
    constraint_values = [
        "@platforms//os:linux",
        "@platforms//cpu:arm",
    ],
)
</pre>

<h3 id="platform_flags">Platform Flags</h3>
<p>
  Platforms may use the <code>flags</code> attribute to specify a list of flags that will be added
  to the configuration whenever the platform is used as the target platform (i.e., as the value of
  the <code>--platforms</code> flag).
</p>

<p>
  Flags set from the platform effectively have the highest precedence and overwrite any previous
  value for that flag, from the command line, rc file, or transition.
</p>

<h4 id="platform_flags_examples">Example</h4>

<pre class="code">
platform(
    name = "foo",
    flags = [
        "--dynamic_mode=fully",
        "--//bool_flag",
        "--no//package:other_bool_flag",
    ],
)
</pre>

<p>
  This defines a platform named <code>foo</code>. When this is the target platform (either because
  the user specified <code>--platforms//:foo</code>, because a transition set the
  <code>//command_line_option:platforms</code> flag to <code>["//:foo"]</code>, or because
  <code>//:foo</code> was used as an execution platform), then the given flags will be set in the
  configuration.
</p>

<h4 id=platform_flags_repeated>Platforms and Repeatable Flags</h4>

<p>
  Some flags will accumulate values when they are repeated, such as <code>--features</code>,
  <code>--copt</code>, any Starlark flag created as <code>config.string(repeatable = True)</code>.
  These flags are not compatible with setting the flags from the platform: instead, all previous
  values will be removed and overwritten with the values from the platform.
</p>

<p>
  As an example, given the following platform, the invocation <code>build --platforms=//:repeat_demo
  --features feature_a --features feature_b</code> will end up with the value of the
  <code>--feature</code> flag being <code>["feature_c", "feature_d"]</code>, removing the features
  set on the command line.
</p>

<pre class="code">
platform(
    name = "repeat_demo",
    flags = [
        "--features=feature_c",
        "--features=feature_d",
    ],
)
</pre>

<p>
  For this reason, it is discouraged to use repeatable flags in the <code>flags</code> attribute.
</p>

<h3 id="platform_inheritance">Platform Inheritance</h3>
<p>
  Platforms may use the <code>parents</code> attribute to specify another platform that they will
  inherit constraint values from. Although the <code>parents</code> attribute takes a list, no
  more than a single value is currently supported, and specifying multiple parents is an error.
</p>

<p>
  When checking for the value of a constraint setting in a platform, first the values directly set
  (via the <code>constraint_values</code> attribute) are checked, and then the constraint values on
  the parent. This continues recursively up the chain of parent platforms. In this manner, any
  values set directly on a platform will override the values set on the parent.
</p>

<p>
  Platforms inherit the <code>exec_properties</code> attribute from the parent platform.
  The dictionary entries in <code>exec_properties</code> of the parent and child platforms
  will be combined.
  If the same key appears in both the parent's and the child's <code>exec_properties</code>,
  the child's value will be used. If the child platform specifies an empty string as a value, the
  corresponding property will be unset.
</p>

<h4 id="platform_inheritance_examples">Example: Constraint Values</h4>
<pre class="code">
platform(
    name = "parent",
    constraint_values = [
        "@platforms//os:linux",
        "@platforms//cpu:arm",
    ],
)
platform(
    name = "child_a",
    parents = [":parent"],
    constraint_values = [
        "@platforms//cpu:x86_64",
    ],
)
platform(
    name = "child_b",
    parents = [":parent"],
)
</pre>

<p>
  In this example, the child platforms have the following properties:

  <ul>
    <li>
      <code>child_a</code> has the constraint values <code>@platforms//os:linux</code> (inherited
      from the parent) and <code>@platforms//cpu:x86_64</code> (set directly on the platform).
    </li>
    <li>
      <code>child_b</code> inherits all constraint values from the parent, and doesn't set any of
      its own.
    </li>
  </ul>
</p>

<h4 id="platform_inheritance_exec_examples">Example: Execution properties</h4>
<pre class="code">
platform(
    name = "parent",
    exec_properties = {
      "k1": "v1",
      "k2": "v2",
    },
)
platform(
    name = "child_a",
    parents = [":parent"],
)
platform(
    name = "child_b",
    parents = [":parent"],
    exec_properties = {
      "k1": "child"
    }
)
platform(
    name = "child_c",
    parents = [":parent"],
    exec_properties = {
      "k1": ""
    }
)
platform(
    name = "child_d",
    parents = [":parent"],
    exec_properties = {
      "k3": "v3"
    }
)
</pre>

<p>
  In this example, the child platforms have the following properties:

  <ul>
    <li>
      <code>child_a</code> inherits the "exec_properties" of the parent and does not set its own.
    </li>
    <li>
      <code>child_b</code> inherits the parent's <code>exec_properties</code> and overrides the
      value of <code>k1</code>. Its <code>exec_properties</code> will be:
      <code>{ "k1": "child", "k2": "v2" }</code>.
    </li>
    <li>
      <code>child_c</code> inherits the parent's <code>exec_properties</code> and unsets
      <code>k1</code>. Its <code>exec_properties</code> will be:
      <code>{ "k2": "v2" }</code>.
    </li>
    <li>
      <code>child_d</code> inherits the parent's <code>exec_properties</code> and adds a new
      property. Its <code>exec_properties</code> will be:
      <code>{ "k1": "v1",  "k2": "v2", "k3": "v3" }</code>.
    </li>
  </ul>
</p>

(Â
	toolchain˙

name(

aspect_hints

deprecation

exec_compatible_with


features


licenses

package_metadata

tags

target_compatible_with

target_settings


testonly

	toolchain(

toolchain_type(
!
use_target_platform_constraints


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies


distribs"ÿ
<p>This rule declares a specific toolchain's type and constraints so that it can be selected
during toolchain resolution. See the
<a href="https://bazel.build/docs/toolchains">Toolchains</a> page for more
details.

(û
toolchain_typeß

name(

aspect_hints

compatible_with

deprecation


features

no_match_error

package_metadata

restricted_to

tags

target_compatible_with


testonly


visibility

transitive_configs

generator_name

generator_function

generator_location

$config_dependencies"ﬂ
<p>
  This rule defines a new type of toolchain -- a simple target that represents a class of tools that
  serve the same role for different platforms.
</p>

<p>
  See the <a href="/docs/toolchains">Toolchains</a> page for more details.
</p>

<h4 id="toolchain_type_examples">Example</h4>
<p>
  This defines a toolchain type for a custom rule.
</p>
<pre class="code">
toolchain_type(
    name = "bar_toolchain_type",
)
</pre>

<p>
  This can be used in a bzl file.
</p>
<pre class="code">
bar_binary = rule(
    implementation = _bar_binary_impl,
    attrs = {
        "srcs": attr.label_list(allow_files = True),
        ...
        # No `_compiler` attribute anymore.
    },
    toolchains = ["//bar_tools:toolchain_type"]
)
</pre>
(