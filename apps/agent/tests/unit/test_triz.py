"""Unit tests for deterministic TRIZ lookup — no LLM involved."""

import pytest

from app.triz import (
    PARAMETERS,
    PRINCIPLES,
    get_parameter_name,
    get_principle_name,
    lookup_principles,
)


def test_parameters_count():
    assert len(PARAMETERS) == 39


def test_principles_count():
    assert len(PRINCIPLES) == 40


def test_parameter_names_numbered_1_to_39():
    for n in range(1, 40):
        assert n in PARAMETERS
        assert isinstance(PARAMETERS[n], str)
        assert len(PARAMETERS[n]) > 0


def test_principle_names_numbered_1_to_40():
    for n in range(1, 41):
        assert n in PRINCIPLES
        assert isinstance(PRINCIPLES[n], str)
        assert len(PRINCIPLES[n]) > 0


def test_lookup_returns_list():
    result = lookup_principles(27, 7)
    assert isinstance(result, list)


def test_lookup_oil_spills_key_cell():
    """Row 27 (Reliability) x Col 7 (Volume moving) -- core oil spills contradiction."""
    principles = lookup_principles(27, 7)
    assert len(principles) >= 1
    for p in principles:
        assert 1 <= p <= 40, f"Principle {p} out of range 1-40"


def test_lookup_principles_in_valid_range():
    """All matrix cells must contain principle numbers 1-40."""
    from app.triz import MATRIX

    for improving, row in MATRIX.items():
        for worsening, principles in row.items():
            for p in principles:
                assert 1 <= p <= 40, (
                    f"Invalid principle {p} at [{improving}][{worsening}]"
                )


def test_lookup_same_param_returns_empty():
    """Diagonal cells (improving == worsening) must be empty."""
    for n in range(1, 40):
        assert lookup_principles(n, n) == []


def test_lookup_invalid_param_raises():
    with pytest.raises(ValueError):
        lookup_principles(0, 7)
    with pytest.raises(ValueError):
        lookup_principles(27, 40)
    with pytest.raises(ValueError):
        lookup_principles(-1, 5)


def test_get_parameter_name_known():
    assert get_parameter_name(27) == "Reliability"


def test_get_parameter_name_unknown():
    name = get_parameter_name(99)
    assert "99" in name


def test_get_principle_name_known():
    assert get_principle_name(35) == "Parameter changes"


def test_lookup_reliability_vs_harmful_effects():
    """Reliability vs Loss of substance — also relevant to oil spills."""
    principles = lookup_principles(27, 23)
    assert isinstance(principles, list)
    for p in principles:
        assert 1 <= p <= 40
