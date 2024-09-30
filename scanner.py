from types import ModuleType
import inspect
import os


class Scanner:
    scanned = set()
    classes = {}

    BLACKLIST = [
        "has_attribute",
        "__subclasshook__",
        "__class__",
        "__dir__",
        "__doc__",
        "__format__",
        "__getattribute__",
        "__init__",
        "__new__",
        "__init_subclass__",
        "__sizeof__",
        "__setattr__",
        "__reduce__",
        "__reduce_ex__",
        "__module__",
        "__getstate__",
        "__delattr__",
        "__ge__",
        "__gt__",
        "__le__",
        "__ne__",
        "__repr__",
        "__str__",
    ]

    def scan_module(self, root_namespace: str, module: ModuleType):
        for element_name in dir(module):
            if element_name.startswith("__"):
                continue

            if root_namespace and root_namespace in module.__name__:
                continue

            if element_name in self.scanned:
                continue

            self.scanned.add(element_name)

            module_root = (
                f"{root_namespace + ('.' if root_namespace else '')}{module.__name__}"
            )
            class_obj = eval(f"{module_root}.{element_name}")

            if isinstance(class_obj, ModuleType):
                self.scan_module(module_root, class_obj)
                continue

            class_members_all = inspect.getmembers(class_obj)
            dunder_members = [
                m
                for m in class_members_all
                if m[0].startswith("__") and m[0] not in Scanner.BLACKLIST
            ]
            class_members = [
                m
                for m in class_members_all
                if not m[0].startswith("__") and m[0] not in Scanner.BLACKLIST
            ]

            class_methods = []
            class_constants = []
            class_attributes = []
            for m in class_members:
                if callable(m[1]):
                    class_methods.append(m)
                elif inspect.isdatadescriptor(m[1]):
                    class_attributes.append(m)
                else:
                    class_constants.append(m)
            for m in dunder_members:
                class_methods.append(m)

            class_definition = (
                class_obj,
                class_constants,
                class_attributes,
                class_methods,
            )

            self.classes.setdefault(module_root, list())
            self.classes[module_root].append(class_definition)

    def write_pyis(self, pyi_root):
        for module, classes in reversed(self.classes.items()):
            print(f"Writing pyi for {module}...")
            path = module.replace(".", "/")
            path = os.path.basename(path)
            path = f"{pyi_root}/{path}.pyi"
            os.makedirs(os.path.dirname(f"{pyi_root}"), exist_ok=True)
            with open(path, "w") as file:
                file.write(f"from typing import Final\n\n")
                for (
                    class_obj,
                    class_constants,
                    class_attributes,
                    class_methods,
                ) in classes:
                    doc = inspect.getdoc(class_obj)
                    if doc:
                        file.write('"""\n')
                        file.write(doc)
                        file.write('"""\n')

                    file.write(f"class {class_obj.__name__}:\n")

                    for const in class_constants:
                        file.write(
                            f"    {const[0]}: Final[{const[1].__class__.__name__}]\n"
                        )

                    for attr in class_attributes:
                        doc = inspect.getdoc(method[1])
                        if doc:
                            file.write(f'    """\n')
                            file.write(doc)
                            file.write(f'    """\n')
                        file.write(f"    @property\n")
                        file.write(f"    def {attr[0]}(self): ...\n")

                    # always put new first if present
                    class_methods.sort(
                        reverse=True,
                        key=lambda m: "aaaaa" if m[0] == "__new__" else m[0],
                    )

                    for method in class_methods:
                        if callable(method[1]):
                            sig = inspect.signature(method[1])

                            if sig.return_annotation is not inspect._empty:
                                print(f"RETURN  = {sig.return_annotation}")
                            if "self" not in sig.parameters:
                                file.write("    @staticmethod\n")

                            if not method[0].startswith("__"):
                                doc = inspect.getdoc(method[1])
                                if doc:
                                    file.write(f'    """\n')
                                    file.write(doc)
                                    file.write("\n")
                                    file.write(f'    """\n')
                            file.write(
                                f"    def {method[0]}({', '.join([str(p) for p in sig.parameters.values()])})"
                            )
                            if method[0] == "__new__":
                                file.write(f" -> {class_obj.__name__}")
                            file.write(f": ...\n")

                    file.write("\n")


Scanner()
