#!/usr/bin/env python

import math
import os
import pickle
import re
import shutil
import subprocess as sp
import time
from datetime import datetime, timedelta
from pathlib import Path
from typing import NamedTuple


class Result(NamedTuple):
    seed: str
    filename: str
    score: int
    time: datetime

    def desc(self):
        # TODO: テストケースを紹介する変数を追加すべき
        return f"{self.filename} (TODO: add test case description) ... score = {self.score}"


def main():
    script_dir = Path(__file__).absolute().parent
    os.chdir(script_dir)

    bin_dir = script_dir / "tools" / "target" / "release"
    bin_gen = bin_dir / "gen"
    bin_vis = bin_dir / "vis"

    in_dir = Path("in")
    out_dir = Path("out")
    MIN_RESULTS_JSON = Path("min_results.bin")

    if Path(in_dir).exists():
        shutil.rmtree(in_dir)
    sp.check_output([bin_gen, "seeds.txt"])

    if Path(out_dir).exists():
        shutil.rmtree(out_dir)
    os.makedirs(out_dir)

    in_filenames = os.listdir(in_dir)

    with open("seeds.txt", encoding="utf-8") as f:
        seeds = [s.strip() for s in f.readlines()]

    sp.check_call(["cargo", "build", "--release"])

    results: list[Result] = []
    for i, filename in enumerate(in_filenames):
        print(f"testing {filename}... ({i+1}/{len(in_filenames)})\r", end="")
        no = int(Path(filename).stem)
        seed = seeds[no]

        in_file_path = in_dir / filename
        out_file_path = out_dir / filename
        err_file_path = out_dir / (filename + ".stderr")

        with open(in_file_path, "r", encoding="utf-8") as f:
            n, m, d, k = map(int, f.readline().split())

        with open(in_file_path, "r", encoding="utf-8") as in_file, open(
            out_file_path, "w", encoding="utf-8", newline="\n"
        ) as out_file, open(
            err_file_path, "w", encoding="utf-8", newline="\n"
        ) as err_file:
            # print("testing for", filename, f"({m}, {eps})", end="... ")
            start_time = time.time()
            sp.check_call(
                ["../target/release/main.exe"],
                stdin=in_file,
                stdout=out_file,
                stderr=err_file,
            )
            end_time = time.time()
            sp.check_call(
                [bin_vis, in_file_path, out_file_path], stdout=err_file
            )
            print(f"time: {end_time - start_time} secs", file=err_file)
        with open(err_file_path, "r", encoding="utf-8") as f:
            res = f.read()
            score = (
                int(cap.group(1))
                if (cap := re.search(r"Score = (\d*)", res))
                else int(1e9)
            )

            results.append(
                Result(
                    filename=filename,
                    seed=seed,
                    score=score,
                    time=datetime.now(),
                )
            )

    past_min_results: dict[str, list[Result]] = {}
    if Path(MIN_RESULTS_JSON).exists():
        with open(MIN_RESULTS_JSON, "rb") as f:
            past_min_results = pickle.load(f)
    results.sort(key=lambda r: r.score)

    print("# Results")
    print()
    for result in results:
        is_new_record = (
            result.seed not in past_min_results
            or past_min_results[result.seed][-1].score > result.score
        )
        if is_new_record:
            if result.seed not in past_min_results:
                past_min_results[result.seed] = []
            past_min_results[result.seed].append(result)

        desc = result.desc()
        if is_new_record:
            desc += " (new record)"
        else:
            past_score = past_min_results[result.seed][-1].score
            desc += f" ({result.score - past_score:+}, {past_score / result.score})"
        print(desc)

    with open(MIN_RESULTS_JSON, "wb") as f:
        pickle.dump(past_min_results, f)

    # 少し前のデータと比較する
    until = datetime.now() + timedelta(days=-1)
    print()
    print(f"# relative scores until {until}")
    print()
    score_sum = 0
    rel_score_sum = 0
    for result in results:
        score_sum += result.score

        # until 以前の最強データと比較する
        past_score = math.nan
        rel_score = math.nan
        for past_result in past_min_results[result.seed][::-1]:
            if past_result.time <= until:
                # until 以前の最後のスコア = 最強のスコア
                past_score = past_result.score
                rel_score = past_score / result.score
                break
        desc = result.desc()
        desc += f" ({result.score - past_score:+}: {rel_score})"
        print(desc)
        # NaN なら rel_score_sum が NaN になってくれるはず
        rel_score_sum += rel_score

    print(f"total score: {score_sum}, rel score: {rel_score_sum}")


if __name__ == "__main__":
    main()
