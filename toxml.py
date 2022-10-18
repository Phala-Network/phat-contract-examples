#!/usr/bin/env python

import sys
import json

from xml.dom import minidom
from xml.etree import cElementTree as ET


def toxml(f):
    data = json.load(f)
    root = ET.Element('program')
    for item in data['items']:
        root.append(node(item))
    return root


def node(item):
    n = ET.Element('node')
    for (k, dk) in [
            ("shallow_size", "ss"),
            ("shallow_size_percent", "ssp"),
            ("retained_size", "rs"),
            ("retained_size_percent", "rsp"),
            ("name", "z")]:
        if item.get(k) is not None:
            if 'percent' in k:
                n.attrib[dk] = "{:.3f}%".format(item.get(k))
            else:
                n.attrib[dk] = str(item.get(k))
    for kid in item.get("children") or []:
        n.append(node(kid))
    return n


def pretty_print_xml(xml):
    text = ET.tostring(xml, encoding='UTF-8')
    return minidom.parseString(text).toprettyxml(encoding='UTF-8')


if __name__ == '__main__':
    xml = toxml(sys.stdin)
    print(pretty_print_xml(xml))


