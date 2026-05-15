#include "gtest/gtest.h"

TEST(MathTest, Addition) {
    EXPECT_EQ(2 + 2, 4);
}

TEST(MathTest, Subtraction) {
    EXPECT_EQ(5 - 3, 2);
}

TEST(StringTest, Length) {
    EXPECT_EQ(std::string("hello").size(), 5u);
}

TEST(StringTest, Empty) {
    EXPECT_TRUE(std::string().empty());
}

TEST(EdgeCaseTest, SingleTest) {
    EXPECT_TRUE(true);
}
