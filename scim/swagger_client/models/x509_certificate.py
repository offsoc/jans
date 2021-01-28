# coding: utf-8

"""
    SCIM API

    Janssen SCIM 2.0 server API. Developers can think of SCIM as a REST API with endpoints exposing CRUD functionality (create, update, retrieve and delete) for identity management resources such as users, groups, and fido devices.   # noqa: E501

    OpenAPI spec version: 5.0.0
    
    Generated by: https://github.com/swagger-api/swagger-codegen.git
"""

import pprint
import re  # noqa: F401

import six

class X509Certificate(object):
    """NOTE: This class is auto generated by the swagger code generator program.

    Do not edit the class manually.
    """
    """
    Attributes:
      swagger_types (dict): The key is attribute name
                            and the value is attribute type.
      attribute_map (dict): The key is attribute name
                            and the value is json key in definition.
    """
    swagger_types = {
        'value': 'str',
        'display': 'str',
        'type': 'str',
        'primary': 'bool'
    }

    attribute_map = {
        'value': 'value',
        'display': 'display',
        'type': 'type',
        'primary': 'primary'
    }

    def __init__(self, value=None, display=None, type=None, primary=None):  # noqa: E501
        """X509Certificate - a model defined in Swagger"""  # noqa: E501
        self._value = None
        self._display = None
        self._type = None
        self._primary = None
        self.discriminator = None
        if value is not None:
            self.value = value
        if display is not None:
            self.display = display
        if type is not None:
            self.type = type
        if primary is not None:
            self.primary = primary

    @property
    def value(self):
        """Gets the value of this X509Certificate.  # noqa: E501

        DER-encoded X.509 certificate  # noqa: E501

        :return: The value of this X509Certificate.  # noqa: E501
        :rtype: str
        """
        return self._value

    @value.setter
    def value(self, value):
        """Sets the value of this X509Certificate.

        DER-encoded X.509 certificate  # noqa: E501

        :param value: The value of this X509Certificate.  # noqa: E501
        :type: str
        """

        self._value = value

    @property
    def display(self):
        """Gets the display of this X509Certificate.  # noqa: E501


        :return: The display of this X509Certificate.  # noqa: E501
        :rtype: str
        """
        return self._display

    @display.setter
    def display(self, display):
        """Sets the display of this X509Certificate.


        :param display: The display of this X509Certificate.  # noqa: E501
        :type: str
        """

        self._display = display

    @property
    def type(self):
        """Gets the type of this X509Certificate.  # noqa: E501


        :return: The type of this X509Certificate.  # noqa: E501
        :rtype: str
        """
        return self._type

    @type.setter
    def type(self, type):
        """Sets the type of this X509Certificate.


        :param type: The type of this X509Certificate.  # noqa: E501
        :type: str
        """

        self._type = type

    @property
    def primary(self):
        """Gets the primary of this X509Certificate.  # noqa: E501

        Denotes if this is the preferred certificate among others, if any  # noqa: E501

        :return: The primary of this X509Certificate.  # noqa: E501
        :rtype: bool
        """
        return self._primary

    @primary.setter
    def primary(self, primary):
        """Sets the primary of this X509Certificate.

        Denotes if this is the preferred certificate among others, if any  # noqa: E501

        :param primary: The primary of this X509Certificate.  # noqa: E501
        :type: bool
        """

        self._primary = primary

    def to_dict(self):
        """Returns the model properties as a dict"""
        result = {}

        for attr, _ in six.iteritems(self.swagger_types):
            value = getattr(self, attr)
            if isinstance(value, list):
                result[attr] = list(map(
                    lambda x: x.to_dict() if hasattr(x, "to_dict") else x,
                    value
                ))
            elif hasattr(value, "to_dict"):
                result[attr] = value.to_dict()
            elif isinstance(value, dict):
                result[attr] = dict(map(
                    lambda item: (item[0], item[1].to_dict())
                    if hasattr(item[1], "to_dict") else item,
                    value.items()
                ))
            else:
                result[attr] = value
        if issubclass(X509Certificate, dict):
            for key, value in self.items():
                result[key] = value

        return result

    def to_str(self):
        """Returns the string representation of the model"""
        return pprint.pformat(self.to_dict())

    def __repr__(self):
        """For `print` and `pprint`"""
        return self.to_str()

    def __eq__(self, other):
        """Returns true if both objects are equal"""
        if not isinstance(other, X509Certificate):
            return False

        return self.__dict__ == other.__dict__

    def __ne__(self, other):
        """Returns true if both objects are not equal"""
        return not self == other
